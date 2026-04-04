use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_industry::StructureUuid;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::industry::{BlueprintBonus, BlueprintTyp, CalculationEngine, Dependency, ProjectConfig, ProjectConfigBuilder, StockMinimal, StructureMapping};
use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use crate::{sort_by_job_flat, sort_by_market_group_flat};
use crate::project::service::ProjectJobStatus;

pub async fn split_job_check(
    pool:                       &PgPool,
    character_id:               CharacterId,
    eve_gateway_api_client:     &impl EveGatewayApiClient,
    project_id:                 ProjectUuid,
    split:                      SplitJobRequest,
) -> Result<SplitJobResponse> {
    let project = crate::project::service::fetch(
            pool,
            character_id,
            project_id,
            eve_gateway_api_client
        )
        .await?
        .ok_or(ProjectError::NotFound(project_id))?;

    let solution_id = if let Some(x) = project.solution_id {
        x
    } else {
        return Err(ProjectError::NoIndustryHub);
    };

    let industry_hub_id = sqlx::query!("
            SELECT industry_hub_id
            FROM solution
            WHERE id = $1
        ",
            *solution_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|_| ProjectError::SolutionNotFound)?
        .industry_hub_id
        .into();

    let industry_hub = crate::industry_hub::service::fetch(
            pool,
            eve_gateway_api_client,
            character_id,
            industry_hub_id,
        )
        .await?
        .ok_or(ProjectError::NoIndustryHub)?;

    let blacklist_type_ids = project
        .project_group
        .blacklist(pool, eve_gateway_api_client)
        .await?
        .into_iter()
        .map(|x| x.type_id)
        .collect::<Vec<_>>();

    let mut overwrites = project
        .project_group
        .overwrites(pool, eve_gateway_api_client)
        .await?
        .into_iter()
        .map(|x| (x.item.type_id, BlueprintBonus::new(x.item.type_id, x.material_efficiency as f32, 0f32)))
        .collect::<HashMap<_, _>>();
    let products = project
        .products(pool, eve_gateway_api_client)
        .await?
        .into_iter()
        .map(|x| (x.item.type_id, BlueprintBonus::new(x.item.type_id, x.material_efficiency as f32, 0f32)))
        .collect::<HashMap<_, _>>();
    overwrites.extend(products);

    let max_runs = project
        .project_group
        .job_splitting(pool, eve_gateway_api_client)
        .await?
        .runs
        .into_iter()
        .map(|x| (x.item.type_id, x.max_runs as u32))
        .collect::<HashMap<_, _>>();

    let mut excess = project
        .excess(
            pool,
            eve_gateway_api_client,
        )
        .await?
        .into_iter()
        .map(|x| StockMinimal {
            quantity:   x.quantity,
            type_id:    x.item.type_id,
        })
        .collect::<Vec<_>>();

    let jobs = project
        .jobs(
                pool,
                character_id,
                eve_gateway_api_client
        )
        .await?
        .into_iter()
        .flat_map(|x| x.entries)
        .filter(|x| x.status == ProjectJobStatus::Done)
        .map(|x| StockMinimal {
            quantity:   x.runs,
            type_id:    x.item.type_id,
        })
        .collect::<Vec<_>>();
    excess.extend(jobs);

    let structure_mappings = industry_hub
        .structures
        .iter()
        .map(|x| StructureMapping {
            category_group: x.joined_categories_groups(),
            structure_uuid: x.id,
        })
        .collect::<Vec<_>>();

    let dependency_json = eve_gateway_api_client
        .fetch_blueprint_json(split.old.type_id)
        .await?
        .unwrap();

    let project_config = ProjectConfigBuilder::default()
        .add_structures(industry_hub.structures)
        .add_structure_mappings(structure_mappings)
        .add_blacklists(blacklist_type_ids)
        .add_blueprint_overwrites(overwrites)
        .set_max_runs(max_runs)
        .build();

    let mut engine_old = CalculationEngine::new(project_config.clone());
    let mut engine_new = CalculationEngine::new(project_config.clone());

    // add the dependencies to the engines
    let dependency = Dependency::try_from(split.old.runs, dependency_json.data.clone()).unwrap();
    engine_old.add(dependency);

    for job in split.new.iter() {
        let dependency = Dependency::try_from(job.runs as u32, dependency_json.data.clone()).unwrap();
        engine_new.add(dependency);
    }

    // generate solutions for the change
    let mut tree_old = engine_old;
    let dependency_result_old = tree_old
        .apply_bonus()
        .add_stocks(&excess)
        .finalize();

    let mut tree_new = engine_new;
    let dependency_result_new = tree_new
        .apply_bonus()
        .add_stocks(&excess)
        .finalize();

    // collect all materials together
    let mut needed_materials_old = HashMap::new();
    dependency_result_old
        .tree
        .into_iter()
        .filter(|(_, x)| x.is_product)
        .flat_map(|(_, x)| x.children)
        .for_each(|(type_id, materials)| {
            needed_materials_old
                .entry(type_id)
                .and_modify(|x: &mut i32| *x += (materials * split.old.runs as f32).ceil() as i32)
                .or_insert((materials * split.old.runs as f32).ceil() as i32);
        });

    let mut needed_materials_new = HashMap::new();
    dependency_result_new
        .tree
        .iter()
        .filter(|(_, x)| x.is_product)
        .flat_map(|(_, x)| x.children.clone())
        .for_each(|(type_id, materials)| {
            for entry in split.new.iter() {
                needed_materials_new
                    .entry(type_id)
                    .and_modify(|x: &mut i32| *x += (materials * entry.runs as f32).ceil() as i32)
                    .or_insert((materials * entry.runs as f32).ceil() as i32);
            }
        });

    // create a diff between both the old and new solution
    let mut material_diff = HashMap::new();
    let mut product_diff = HashMap::new();
    for (type_id, materials) in needed_materials_new {
        let old_materials = if let Some(x) = needed_materials_old.get(&type_id) {
            x
        } else {
            material_diff.insert(type_id, materials);
            continue;
        };

        if materials - old_materials == 0 {
            continue;
        } else {
            if let Some(x) = dependency_result_new
                .tree
                .get(&type_id) {

                if x.typ == BlueprintTyp::Blueprint ||  x.typ == BlueprintTyp::Reaction {
                    product_diff.insert(type_id, materials - old_materials);
                    continue;
                }
            }

            material_diff.insert(type_id, materials - old_materials);
        }
    }

    let (sub_materials, sub_jobs, sub_excess) = determine_dependant_materials(
            eve_gateway_api_client,
            &project_config,
            product_diff,
            &excess,
        )
        .await?;

    let mut type_ids = material_diff
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    type_ids.extend(sub_materials.keys().collect::<Vec<_>>());
    type_ids.extend(sub_jobs.keys().collect::<Vec<_>>());
    type_ids.extend(sub_excess.keys().collect::<Vec<_>>());
    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let mut result_materials = Vec::new();
    for (type_id, quantity) in material_diff {
        if quantity == 0 {
            continue;
        }

        let item = if let Some(x) = items.get(&type_id) {
            x.clone()
        } else {
            continue;
        };

        result_materials.push(SplitJobResponseMarketEntry {
            item,
            quantity,
        });
    }
    for (type_id, quantity) in sub_materials {
        if quantity == 0 {
            continue;
        }

        let item = if let Some(x) = items.get(&type_id) {
            x.clone()
        } else {
            continue;
        };

        result_materials.push(SplitJobResponseMarketEntry {
            item,
            quantity,
        });
    }

    let mut result_excess = Vec::new();
    for (type_id, quantity) in sub_excess {
        if quantity == 0 {
            continue;
        }

        let item = if let Some(x) = items.get(&type_id) {
            x.clone()
        } else {
            continue;
        };

        result_excess.push(SplitJobResponseMarketEntry {
            item,
            quantity,
        });
    }

    let mut result_jobs = Vec::new();
    for (type_id, (runs, structure_id)) in sub_jobs {
        if runs == 0 {
            continue;
        }

        let item = if let Some(x) = items.get(&type_id) {
            x.clone()
        } else {
            continue;
        };

        result_jobs.push(SplitJobResponseJobEntry {
            item,
            runs,
            structure_id,
        });
    }

    Ok(SplitJobResponse {
        jobs:       sort_jobs(result_jobs),
        materials:  sort_materials(result_materials),
        excess:     sort_materials(result_excess),
    })
}

async fn determine_dependant_materials(
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_config:         &ProjectConfig,
    additional_products:    HashMap<TypeId, i32>,
    stocks:                 &Vec<StockMinimal>,
) -> Result<(HashMap<TypeId, i32>, HashMap<TypeId, (i32, StructureUuid)>, HashMap<TypeId, i32>)> {
    let mut engine = CalculationEngine::new(project_config.clone());

    for (type_id, runs) in additional_products {
        let dependency_json = eve_gateway_api_client
            .fetch_blueprint_json(type_id)
            .await?
            .unwrap();
        let dependency = Dependency::try_from(runs as u32, dependency_json.data.clone()).unwrap();
        engine.add(dependency);
    }

    let mut tree = engine;
    let dependency_result = tree
        .apply_bonus()
        .add_stocks(stocks)
        .finalize();

    let mut needed_materials = HashMap::new();
    let mut needed_jobs = HashMap::new();
    let mut excess = HashMap::new();

    dependency_result
        .tree
        .iter()
        .for_each(|(type_id, entry)| {
            if entry.typ == BlueprintTyp::Blueprint || entry.typ == BlueprintTyp::Reaction {
                let runs: u32 = entry.runs.iter().sum();
                needed_jobs
                    .entry(*type_id)
                    .or_insert((runs as i32, entry.structure.as_ref().unwrap().id));

                let total_produced: u32 = entry
                .runs
                    .iter()
                    .map(|x| x * entry.produces as u32)
                    .sum();

                let excess_quantity = total_produced.saturating_sub(entry.needed.ceil() as u32);
                if excess_quantity != 0 {
                    excess
                        .entry(entry.product_type_id)
                        .or_insert(excess_quantity as i32);
                }
            } else {
                needed_materials
                    .entry(*type_id)
                    .or_insert((entry.needed).ceil() as i32);
            }
        });

    Ok((needed_materials, needed_jobs, excess))
}

sort_by_job_flat!(sort_jobs, SplitJobResponseJobEntry);
sort_by_market_group_flat!(sort_materials, SplitJobResponseMarketEntry);

#[derive(Debug, Deserialize, ToSchema)]
pub struct SplitJobRequest {
    pub old: SplitJobEntry,
    pub new: Vec<SplitJobEntry>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct SplitJobEntry {
    pub type_id: TypeId,
    pub runs:    u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SplitJobResponse {
    pub excess:     Vec<SplitJobResponseMarketEntry>,
    pub materials:  Vec<SplitJobResponseMarketEntry>,
    pub jobs:       Vec<SplitJobResponseJobEntry>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct SplitJobResponseJobEntry {
    pub item:           Item,
    pub runs:           i32,
    pub structure_id:   StructureUuid,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct SplitJobResponseMarketEntry {
    pub item:       Item,
    pub quantity:   i32,
}
