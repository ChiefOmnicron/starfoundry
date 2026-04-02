mod engine;
mod models;
mod project_config_builder;
mod project_config;
mod result;

pub use self::engine::*;
pub use self::models::*;
pub use self::project_config::*;
pub use self::project_config_builder::*;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::{EveGatewayApiClientIndustry, EveGatewayApiClientItem, Item};
use starfoundry_lib_gateway::{ErrorResponse, ExtractIdentity};
use starfoundry_lib_industry::Structure;
use starfoundry_lib_market::MarketApiClientPrice;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::{AppState, eve_gateway_api_client, market_api_client, sort_by_job_flat, sort_by_market_group_flat};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::industry::error::Result;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::{list_default_blacklist, list_default_blueprint_overwrite, list_default_job_splitting, list_industry_hubs};
use crate::industry_hub::service::IndustryHub;
use sqlx::PgPool;
use crate::project::SolutionUuid;
use crate::industry_hub::IndustryHubUuid;
use uuid::Uuid;

// TODO: refactor
/// Build plan
/// 
/// - Alternative route: `/latest/industry/calculation`
/// - Alternative route: `/v1/industry/calculation`
/// 
/// ---
/// 
/// Creates a build solution
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    path = "/calculation",
    tag = "Industry",
    responses(
        (
            body = serde_json::Value,
            description = "Information about the structure",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity:     ExtractIdentity,
    State(state): State<AppState>,
    Json(config): Json<TmpRequest>,
) -> Result<impl IntoResponse> {
    let products = if let Some(x) = config.products {
        x
    } else if let Some(x) = config.products_str {
        eve_gateway_api_client()?
            .parse_items(x)
            .await?
            .items
            .into_iter()
            .map(|x| TmpProductRequest {
                quantity:   x.quantity as u32,
                type_id:    x.type_id,
                material_efficiency: x.material_efficiency.map(|x| x as u32).unwrap_or(10),
            })
            .collect::<Vec<_>>()
    } else {
        return Ok(
            (
                StatusCode::BAD_REQUEST,
                Json(
                    ErrorResponse {
                        error: "INVALID_REQUEST".into(),
                        description: "Either materials or materials_str must be set.".into(),
                    }
                )
            )
            .into_response()
        );
    };

    let stocks = if let Some(x) = config.stocks {
        x
    } else if let Some(x) = config.stocks_str {
        eve_gateway_api_client()?
            .parse_items(x)
            .await?
            .items
            .into_iter()
            .map(|x| StockMinimal {
                quantity:   x.quantity as i32,
                type_id:    x.type_id,
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let industry_hubs = list_industry_hubs(
            &state.postgres,
            &eve_gateway_api_client()?,
            identity.character_id,
            config.project_group_id,
        )
        .await?;

    let blacklist = if let Some(x) = config.blacklist {
        x
    } else {
        list_default_blacklist(
                &state.postgres,
                &eve_gateway_api_client()?,
                config.project_group_id,
            )
            .await?
            .into_iter()
            .map(|x| x.type_id)
            .collect::<Vec<_>>()
    };

    let job_splitting = if let Some(x) = config.job_splitting {
        x
            .into_iter()
            .map(|x| (x.type_id, x.runs as u32))
            .collect::<HashMap<_, _>>()
    } else {
        list_default_job_splitting(
                &state.postgres,
                &eve_gateway_api_client()?,
                config.project_group_id,
            )
            .await?
            .runs
            .into_iter()
            .map(|x| (x.item.type_id, x.max_runs as u32))
            .collect::<HashMap<_, _>>()
    };

    let mut blueprint_overwrites = if let Some(x) = config.blueprint_overwrite {
        x
            .into_iter()
            .map(|x| (x.type_id, BlueprintBonus { material: x.material_efficiency as f32, type_id: x.type_id, time: 0f32}))
            .collect::<HashMap<_, _>>()
    } else {
        list_default_blueprint_overwrite(
                &state.postgres,
                &eve_gateway_api_client()?,
                config.project_group_id,
            )
            .await?
            .into_iter()
            .map(|x| (x.item.type_id, BlueprintBonus { material: x.material_efficiency as f32, type_id: x.item.type_id, time: 0f32}))
            .collect::<HashMap<_, _>>()
    };
    let tmp_blueprint_overwrites = products
        .iter()
        .map(|x| (
            x.type_id,
            BlueprintBonus {
                type_id:    x.type_id,
                material:   x.material_efficiency as f32,
                time:       0f32,
            }
        ))
        .collect::<HashMap<_, _>>();
    blueprint_overwrites.extend(tmp_blueprint_overwrites);

    let market_prices = market_api_client()?
        .all_prices()
        .await?
        .into_iter()
        .map(|x| (x.type_id, x.adjusted_price))
        .collect::<HashMap<_, _>>();

    let mut solutions = Vec::new();
    for industry_hub in industry_hubs {
        let mapping = industry_hub
            .structures
            .iter()
            .map(|x| {
                StructureMapping {
                    category_group: x.joined_categories_groups(),
                    structure_uuid: x.id,
                }
            })
            .collect::<Vec<_>>();

        let mut system_index = HashMap::new();
        let mut all_system_ids = Vec::new();
        for structure in industry_hub.structures.iter() {
            all_system_ids.push(structure.system.system_id);
        }
        all_system_ids.sort();
        all_system_ids.dedup();

        for system_id in all_system_ids {
            let index = &eve_gateway_api_client()?
                .fetch_system_index(system_id)
                .await
                .unwrap()
                .unwrap();
            system_index.insert(
                system_id,
                (
                    index.manufacturing,
                    index.reaction,
                )
            );
        }

        let project_config = ProjectConfigBuilder::default()
            .add_blacklists(blacklist.clone())
            .add_blueprint_overwrites(blueprint_overwrites.clone())
            .add_structures(industry_hub.structures.clone())
            .add_structure_mappings(mapping)
            .set_max_runs(job_splitting.clone())
            .set_material_cost(market_prices.clone())
            .set_system_index(system_index.clone())
            .build();

        let mut dependency_tree = CalculationEngine::new(project_config);

        let eve_gateway_client = eve_gateway_api_client()?;
        for product in products.iter() {
            let dependency = if let Ok(Some(x)) = eve_gateway_client.fetch_blueprint_json(product.type_id).await {
                x.data
            } else {
                continue
            };

            let json = serde_json::to_value(&dependency).unwrap();

            // FIXME: actual quantity
            if let Ok(x) = Dependency::try_from(product.quantity, json) {
                dependency_tree.add(x);
            } else {
                continue;
            };
        }

        let dependency_result = dependency_tree
            .apply_bonus()
            .add_stocks(&stocks)
            .finalize();

        let manufacturing = dependency_result
            .tree
            .iter()
            .filter(|(_, x)| x.typ != BlueprintTyp::Material)
            .map(|(_, x)| TmpManufacturingResponse {
                item:       x.item.clone(),
                build_tax:  x.build_cost.total_job_cost,
                runs:       x.runs.clone(),
                structure:  x.structure.clone(),
                time:       x.time,
            })
            .collect::<Vec<_>>();

        // filter out solutions where not all jobs have a structure assigned
        if manufacturing
            .iter()
            .find(|x| x.structure.is_none())
            .is_some() {
                continue;
        }

        let material = dependency_result
            .tree
            .iter()
            .filter(|(_, x)| x.typ == BlueprintTyp::Material)
            .map(|(_, x)| TmpMaterialResponse {
                item:   x.item.clone(),
                needed: x.needed,
                stock:  x.stock,
            })
            .collect::<Vec<_>>();

        let excess = dependency_result
            .tree
            .iter()
            .filter(|(_, x)| x.typ != BlueprintTyp::Material)
            .map(|(_, entry)| {
                let total_produced: u32 = entry
                    .runs
                    .iter()
                    .map(|x| x * entry.produces as u32)
                    .sum();

                let excess_quantity = total_produced.saturating_sub(entry.needed.ceil() as u32);
                StockMinimal {
                    quantity: excess_quantity as i32,
                    type_id:  entry.product_type_id,
                }
            })
            .collect::<Vec<_>>();

        let solution_id = store(
                &state.postgres,
                industry_hub.id,
                config.project_group_id,
                blacklist.clone(),
                blueprint_overwrites.clone(),
                job_splitting.clone(),
                stocks.clone(),
                products.clone(),
                excess.clone(),
                material.clone(),
                manufacturing.clone(),
            )
            .await
            .unwrap();

        solutions.push(TmpResponse {
            solution_id:    solution_id,
            industry_hub:   industry_hub,
            material:       sort_market(material),
            manufacturing:  sort_jobs(manufacturing),
        });
    }

    Ok(
        (
            StatusCode::OK,
            Json(solutions)
        )
        .into_response()
    )
}

async fn store(
    pool:                   &PgPool,
    industry_hub_id:        IndustryHubUuid,
    project_group_id:       ProjectGroupUuid,

    blacklist:              Vec<TypeId>,
    blueprint_overwrites:   HashMap<TypeId, BlueprintBonus>,
    job_splitting:          HashMap<TypeId, u32>,
    stock:                  Vec<StockMinimal>,
    products:               Vec<TmpProductRequest>,
    excess:                 Vec<StockMinimal>,
    materials:              Vec<TmpMaterialResponse>,
    manufacturing:          Vec<TmpManufacturingResponse>,
) -> Result<SolutionUuid> {
    let mut transaction = pool.begin().await.unwrap();

    let solution_id = sqlx::query!("
            INSERT INTO solution
            (
                industry_hub_id,
                project_group_id
            )
            VALUES ($1, $2)
            RETURNING id
        ",
            *industry_hub_id,
            *project_group_id,
        )
        .fetch_one(&mut *transaction)
        .await
        .unwrap()
        .id;

    sqlx::query!("
            INSERT INTO solution_blacklist
            (
                solution_id,
                type_id
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[]
            )
        ",
            solution_id,
            &blacklist.clone().into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

    sqlx::query!("
            INSERT INTO solution_blueprint_overwrite
            (
                solution_id,
                type_id,
                material_efficiency
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            solution_id,
            &blueprint_overwrites.keys().map(|x| **x).collect::<Vec<_>>(),
            &blueprint_overwrites.values().map(|x| x.material as i32).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

    sqlx::query!("
            INSERT INTO solution_job_split
            (
                solution_id,
                type_id,
                runs
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            solution_id,
            &job_splitting.keys().map(|x| **x).collect::<Vec<_>>(),
            &job_splitting.values().map(|x| *x as i32).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

    sqlx::query!("
            INSERT INTO solution_stock
            (
                solution_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            solution_id,
            &stock.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
            &stock.iter().map(|x| x.quantity).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

    sqlx::query!("
            INSERT INTO solution_product
            (
                solution_id,
                type_id,
                quantity,
                material_efficiency
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::INTEGER[]
            )
        ",
            solution_id,
            &products.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
            &products.iter().map(|x| x.quantity as i32).collect::<Vec<_>>(),
            &products.iter().map(|x| x.material_efficiency as i32).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

    sqlx::query!("
            INSERT INTO solution_excess
            (
                solution_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            solution_id,
            &excess.iter().map(|x| *x.type_id).collect::<Vec<_>>(),
            &excess.iter().map(|x| x.quantity).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

    sqlx::query!("
            INSERT INTO solution_material
            (
                solution_id,
                type_id,
                quantity
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[]
            )
        ",
            solution_id,
            &materials.iter().map(|x| *x.item.type_id).collect::<Vec<_>>(),
            &materials.iter().map(|x| x.needed as i32).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

    let mut type_id = Vec::new();
    let mut runs = Vec::new();
    let mut structure = Vec::new();
    for job in manufacturing {
        let structure_id = if let Some(x) = job.structure.clone() {
            *x.id
        } else {
            Uuid::default()
        };

        for run in job.runs.iter() {
            type_id.push(*job.item.type_id);
            runs.push(*run as i32);
            structure.push(structure_id);
        }
    }

    sqlx::query!("
            INSERT INTO solution_manufacturing
            (
                solution_id,
                type_id,
                runs,
                structure_id
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::UUID[]
            )
        ",
            solution_id,
            &type_id,
            &runs,
            &structure,
        )
        .execute(&mut *transaction)
        .await
        .unwrap();

    transaction.commit().await.unwrap();

    Ok(solution_id.into())
}

sort_by_market_group_flat!(sort_market, TmpMaterialResponse);
sort_by_job_flat!(sort_jobs, TmpManufacturingResponse);

#[derive(Debug, Deserialize)]
pub struct TmpRequest {
    pub project_group_id:       ProjectGroupUuid,
    pub products:               Option<Vec<TmpProductRequest>>,
    pub products_str:           Option<String>,

    pub stocks:                 Option<Vec<StockMinimal>>,
    pub stocks_str:             Option<String>,

    pub blacklist:              Option<Vec<TypeId>>,
    pub blueprint_overwrite:    Option<Vec<TmpBlueprintOverwrite>>,
    pub job_splitting:          Option<Vec<TmpJobSplitting>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct TmpProductRequest {
    pub type_id:                TypeId,
    pub material_efficiency:    u32,
    pub quantity:               u32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TmpBlueprintOverwrite {
    pub type_id:                TypeId,
    pub material_efficiency:    u32,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TmpJobSplitting {
    pub type_id:    TypeId,
    pub runs:       u32,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TmpResponse {
    pub solution_id:    SolutionUuid,
    pub industry_hub:   IndustryHub,
    pub material:       Vec<TmpMaterialResponse>,
    pub manufacturing:  Vec<TmpManufacturingResponse>,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct TmpMaterialResponse {
    pub item:   Item,
    pub needed: f32,
    pub stock:  i32,
    // TODO: add market
}

#[derive(Clone, Debug, Serialize, ToSchema)]
pub struct TmpManufacturingResponse {
    pub item:       Item,
    pub runs:       Vec<u32>,
    pub structure:  Option<Structure>,
    pub build_tax:  f32,
    pub time:       f32,
}
