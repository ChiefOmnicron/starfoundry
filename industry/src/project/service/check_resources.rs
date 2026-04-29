use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_industry::StructureUuid;
use starfoundry_lib_market::MarketApiClient;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::{sort_by_job_flat, sort_by_market_group_flat};
use crate::industry::{CalculationEngine, Dependency, ProjectConfigBuilder, StructureMapping};
use crate::project::error::{ProjectError, Result};
use crate::structure::service::FetchStructureQuery;
use crate::project::ProjectJobUuid;
use uuid::Uuid;

pub async fn check_resources(
    pool:                       &PgPool,
    eve_gateway_api_client:     &impl EveGatewayApiClient,
    market_api_client:          &impl MarketApiClient,
    character_id:               CharacterId,
    job_ids:                    Vec<ProjectJobUuid>,
    materials:                  Vec<Material>,
) -> Result<CheckMaterialsResponse> {
    let mut total_cost           = 0f32;
    let mut required_resources   = HashMap::new();
    let mut required_blueprints: HashMap<TypeId, Vec<u32>>  = HashMap::new();
    let mut grouped_by_structure = HashMap::new();

    sqlx::query!("
            SELECT
                type_id,
                runs,
                structure_id
            FROM project_job
            WHERE id = ANY($1)
        ",
            &job_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListJobs)?
        .iter()
        .for_each(|x| {
            let job_to_start = JobToStart {
                runs:       x.runs,
                type_id:    x.type_id.into(),
            };

            grouped_by_structure
                .entry(StructureUuid::new(x.structure_id))
                .and_modify(|x: &mut Vec<JobToStart>| x.push(job_to_start.clone()))
                .or_insert(vec![job_to_start]);
        });

    for (structure_uuid, jobs) in grouped_by_structure {
        let structure = if let Ok(Some(x)) = crate::structure::service::fetch(
                pool,
                eve_gateway_api_client,
                character_id,
                structure_uuid,
                FetchStructureQuery::default(),
            )
            .await {

            x
        } else {
            continue;
        };

        let structure_mapping = StructureMapping {
            category_group: structure.joined_categories_groups(),
            structure_uuid: structure_uuid,
        };

        let mut system_index = HashMap::new();
        let index = eve_gateway_api_client
            .fetch_system_index(structure.system.system_id)
            .await
            .unwrap()
            .unwrap();
        system_index.insert(
            structure.system.system_id,
            (
                index.manufacturing,
                index.reaction,
            )
        );

        let market_prices = market_api_client
            .all_prices()
            .await?
            .into_iter()
            .map(|x| (x.type_id, x.adjusted_price))
            .collect::<HashMap<_, _>>();

        let project_config = ProjectConfigBuilder::default()
            .add_structures(vec![structure])
            .add_structure_mappings(vec![structure_mapping])
            .set_skip_children(true)
            .set_material_cost(market_prices.clone())
            .set_system_index(system_index.clone())
            .build();

        let mut engine = CalculationEngine::new(project_config);

        let mut blueprint_cache: HashMap<TypeId, serde_json::Value> = HashMap::new();
        for job in jobs {
            let json = if let Some(x) = blueprint_cache.get(&job.type_id) {
                x.clone()
            } else {
                let dependency = if let Ok(Some(x)) = eve_gateway_api_client
                    .fetch_blueprint_json(job.type_id)
                    .await {

                    x.data
                } else {
                    continue
                };

                let json = if let Ok(x) = serde_json::to_value(&dependency) {
                    x
                } else {
                    continue;
                };

                blueprint_cache.insert(job.type_id, json.clone());
                json
            };

            if let Ok(x) = Dependency::try_from(job.runs as u32, json) {
                engine.add(x);
            } else {
                continue;
            };
        }

        let mut tree = engine;
        let dependency_result = tree
            .apply_bonus()
            .finalize();

        total_cost += dependency_result.total_cost();
        for (_, job) in dependency_result.tree {
            for (type_id, materials) in job.children {
                for run in job.runs.iter() {
                    required_resources
                        .entry(type_id)
                        .and_modify(|x: &mut i32| *x += (materials * *run as f32).ceil() as i32)
                        .or_insert((materials * *run as f32).ceil() as i32);
                }
            }

            required_blueprints.insert(
                job.product_type_id,
                job.runs,
            );
        }
    }

    let blueprint_type_ids = required_blueprints
        .iter()
        .map(|(type_id, _)| *type_id)
        .collect::<Vec<_>>();
    let mut type_ids = required_resources
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    type_ids.extend(blueprint_type_ids);
    type_ids.sort();
    type_ids.dedup();
    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<TypeId, Item>>();

    // remove stock
    for resource in materials {
        required_resources
            .entry(resource.type_id)
            .and_modify(|x: &mut i32| *x -= resource.quantity);
    }

    let mut materials = Vec::new();
    for (type_id, quantity) in required_resources {
        let item = if let Some(x) = items.get(&type_id) {
            x.clone()
        } else {
            continue;
        };
        materials.push(CheckMaterialsResponseMaterial {
            item,
            quantity,
        });
    }

    let mut blueprints = Vec::new();
    for (type_id, runs) in required_blueprints {
        let item = if let Some(x) = items.get(&type_id) {
            x.clone()
        } else {
            continue;
        };
        blueprints.push(CheckMaterialsResponseBlueprint {
            id:   Uuid::now_v7(),
            item: item,
            runs: runs,
        });
    }

    Ok(CheckMaterialsResponse {
        job_cost:   total_cost,
        materials:  sort_materials(materials),
        blueprints: sort_blueprints(blueprints),
    })
}

/// Either `materials` or `materials_str` is required
/// If `materials_str` is given, they will be resolved to their type_id and quantity
/// 
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CheckMaterialsRequest {
    pub job_ids:        Vec<ProjectJobUuid>,
    pub materials:      Option<Vec<Material>>,
    pub materials_str:  Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CheckMaterialsResponse {
    pub job_cost:   f32,
    pub materials:  Vec<CheckMaterialsResponseMaterial>,
    pub blueprints: Vec<CheckMaterialsResponseBlueprint>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CheckMaterialsResponseMaterial {
    pub item:     Item,
    pub quantity: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CheckMaterialsResponseBlueprint {
    // only needed for sorting
    #[serde(skip)]
    pub id:       Uuid,
    pub item:     Item,
    pub runs:     Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct Material {
    pub quantity: i32,
    pub type_id:  TypeId,
}

#[derive(Clone, Debug)]
pub struct JobToStart {
    pub runs:           i32,
    pub type_id:        TypeId,
}

sort_by_market_group_flat!(sort_materials, CheckMaterialsResponseMaterial);
sort_by_job_flat!(sort_blueprints, CheckMaterialsResponseBlueprint);
