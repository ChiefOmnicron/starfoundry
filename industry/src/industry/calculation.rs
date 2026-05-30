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
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientIndustry, EveGatewayApiClientItem};
use starfoundry_lib_gateway::{ErrorResponse, ExtractIdentity};
use starfoundry_lib_industry::industry::{BuildEngine, BuildEngineAdditionalProduct, BuildEngineManufacturingResponse, BuildEngineMaterialResponse, BuildEngineProduct, BuildEngineResponse, StockMinimal};
use starfoundry_lib_industry::IndustryHubUuid;
use starfoundry_lib_industry::ProjectGroupUuid;
use starfoundry_lib_industry::SolutionUuid;
use starfoundry_lib_market::{BuyStrategy, MarketApiClientOrder, MarketApiClientPrice, MarketBulkRequest, MarketItemList};
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{AppState, eve_gateway_api_client, market_api_client, sort_by_job_flat, sort_by_market_group_flat};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::industry::error::Result;
use crate::project_group::service::{list_default_blacklist, list_default_blueprint_overwrite, list_default_job_splitting, list_industry_hubs};

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
    request_body = BuildEngine,
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
    Json(config): Json<BuildEngine>,
) -> Result<impl IntoResponse> {
    let products = if let Some(x) = config.products {
        x
    } else if let Some(x) = config.products_str {
        eve_gateway_api_client()?
            .parse_items(x)
            .await?
            .items
            .into_iter()
            .map(|x| BuildEngineProduct {
                quantity:   x.quantity as u32,
                type_id:    x.type_id,
                material_efficiency: x.material_efficiency.map(|x| x as u32).unwrap_or(0),
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
    let additional_products = if let Some(x) = config.additional_products {
        x
    } else if let Some(x) = config.additional_products_str {
        eve_gateway_api_client()?
            .parse_items(x)
            .await?
            .items
            .into_iter()
            .map(|x| BuildEngineAdditionalProduct {
                quantity:   x.quantity as u32,
                type_id:    x.type_id,
                price:      None,
            })
            .collect::<Vec<_>>()
    } else {
        Vec::new()
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
                dbg!("blueprint found");
                x.data
            } else {
                dbg!("blueprint NOT found", product.type_id);
                continue
            };

            let json = serde_json::to_value(&dependency).unwrap();

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
        dbg!(dependency_result.tree.len());

        let manufacturing = dependency_result
            .tree
            .iter()
            .filter(|(_, x)| x.typ != BlueprintTyp::Material)
            .map(|(_, x)| BuildEngineManufacturingResponse {
                // only needed for sorting
                id:         Uuid::now_v7().into(),
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

        let mut material = dependency_result
            .tree
            .iter()
            .filter(|(_, x)| x.typ == BlueprintTyp::Material)
            .filter(|(_, x)| x.needed as i32 > 0i32)
            .map(|(_, x)| BuildEngineMaterialResponse {
                item:   x.item.clone(),
                needed: x.needed,
                stock:  x.stock,
                price:  None,
            })
            .collect::<Vec<_>>();
        if let Some(true) = config.calculate_market_cost {
            let items = material
                .iter()
                .map(|x| MarketItemList {
                    quantity:   x.needed as i32,
                    type_id:    x.item.type_id,
                })
                .collect::<Vec<_>>();

            let market_entries = market_api_client()?
                .bulk_latest_orders(MarketBulkRequest {
                    strategy:   BuyStrategy::MultiBuy,
                    markets:    config.markets.clone().unwrap_or_default(),
                    item_list:  Some(items),
                    ..Default::default()
                })
                .await?;

            for market_entry in market_entries {
                if let Some(x) = material
                    .iter_mut()
                    .find(|x| x.item.type_id == market_entry.type_id) {

                    x.price = Some(market_entry.price);
                }
            }
        }

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

        let mut updated_stocks = dependency_result.stocks.clone();
        let mut additional_products = additional_products.clone();
        for additional_product in additional_products.iter_mut() {
            if let Some(x) = stocks
                .iter()
                .find(|x| x.type_id == additional_product.type_id) {

                let stock_quantity = std::cmp::min(x.quantity, additional_product.quantity as i32);
                additional_product.quantity = additional_product.quantity.saturating_sub(x.quantity as u32);

                updated_stocks.push(StockMinimal {
                    quantity:   stock_quantity,
                    type_id:    x.type_id,
                });
            }
        }
        if let Some(true) = config.calculate_market_cost {
            let items = additional_products
                .iter()
                .map(|x| MarketItemList {
                    quantity:   x.quantity as i32,
                    type_id:    x.type_id,
                })
                .collect::<Vec<_>>();

            let market_entries = market_api_client()?
                .bulk_latest_orders(MarketBulkRequest {
                    strategy:   BuyStrategy::MultiBuy,
                    markets:    config.markets.clone().unwrap_or_default(),
                    item_list:  Some(items),
                    ..Default::default()
                })
                .await?;

            for market_entry in market_entries {
                if let Some(x) = additional_products
                    .iter_mut()
                    .find(|x| x.type_id == market_entry.type_id) {

                    x.price = Some(market_entry.price);
                }
            }
        }

        let solution_id = store_solution(
                &state.postgres,
                industry_hub.id,
                config.project_group_id,
                blacklist.clone(),
                blueprint_overwrites.clone(),
                job_splitting.clone(),
                updated_stocks.clone(),
                products.clone(),
                excess.clone(),
                material.clone(),
                additional_products.clone(),
                manufacturing.clone(),
            )
            .await
            .unwrap();

        solutions.push(BuildEngineResponse {
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

async fn store_solution(
    pool:                   &PgPool,
    industry_hub_id:        IndustryHubUuid,
    project_group_id:       ProjectGroupUuid,

    blacklist:              Vec<TypeId>,
    blueprint_overwrites:   HashMap<TypeId, BlueprintBonus>,
    job_splitting:          HashMap<TypeId, u32>,
    stock:                  Vec<StockMinimal>,
    products:               Vec<BuildEngineProduct>,
    excess:                 Vec<StockMinimal>,
    materials:              Vec<BuildEngineMaterialResponse>,
    additional_materials:   Vec<BuildEngineAdditionalProduct>,
    manufacturing:          Vec<BuildEngineManufacturingResponse>,
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
                quantity,
                cost
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::DOUBLE PRECISION[]
            )
        ",
            solution_id,
            &materials
                .iter()
                .filter(|x| x.needed > 0f32)
                .map(|x| *x.item.type_id).collect::<Vec<_>>(),
            &materials
                .iter()
                .filter(|x| x.needed > 0f32)
                .map(|x| x.needed as i32).collect::<Vec<_>>(),
            &materials
                .iter()
                .filter(|x| x.needed > 0f32)
                .map(|x| x.price.unwrap_or_default()).collect::<Vec<_>>(),
        )
        .execute(&mut *transaction)
        .await
        .unwrap();
    sqlx::query!("
            INSERT INTO solution_material
            (
                solution_id,
                type_id,
                quantity,
                cost
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::DOUBLE PRECISION[]
            )
        ",
            solution_id,
            &additional_materials
                .iter()
                .filter(|x| x.quantity > 0)
                .map(|x| *x.type_id).collect::<Vec<_>>(),
            &additional_materials
                .iter()
                .filter(|x| x.quantity > 0)
                .map(|x| x.quantity as i32).collect::<Vec<_>>(),
            &additional_materials
                .iter()
                .filter(|x| x.quantity > 0)
                .map(|x| x.price.unwrap_or_default()).collect::<Vec<_>>(),
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
            if *run == 0 {
                continue;
            }

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

sort_by_market_group_flat!(sort_market, BuildEngineMaterialResponse);
sort_by_job_flat!(sort_jobs, BuildEngineManufacturingResponse);
