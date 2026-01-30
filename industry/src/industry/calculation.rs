mod engine;
mod models;
mod project_config_builder;
mod project_config;
mod result;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::{EveGatewayApiClientIndustry, Item};
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_market::MarketApiClientPrice;
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;
use std::str::FromStr;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{AppState, eve_gateway_api_client, market_api_client};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::industry::calculation::engine::CalculationEngine;
use crate::industry::calculation::models::{BlueprintBonus, BlueprintTyp, Dependency, StructureMapping};
use crate::industry::calculation::project_config_builder::ProjectConfigBuilder;
use crate::industry::error::Result;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::{fetch, list_default_blacklist, list_default_blueprint_overwrite, list_default_job_splitting, list_industry_hubs};
use starfoundry_lib_industry::Structure;

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
    let project_group = fetch(
            &state.pool,
            &eve_gateway_api_client()?,
            identity.character_id,
            config.project_group_id,
        )
        .await?
        .unwrap();

    let industry_hubs = list_industry_hubs(
            &state.pool,
            &eve_gateway_api_client()?,
            identity.character_id,
            config.project_group_id,
        )
        .await?
        // FIXME:
        .into_iter()
        .find(|x| x.id == Uuid::from_str("019bc88b-50a0-74b1-9e2c-1a4e88ee541e").unwrap().into())
        .unwrap()
        .clone();

    let splitting = list_default_job_splitting(
            &state.pool,
            &eve_gateway_api_client()?,
            config.project_group_id,
        )
        .await?
        .runs
        .into_iter()
        .map(|x| (x.item.type_id, x.max_runs as u32))
        .collect::<HashMap<_, _>>();

    let blacklist = list_default_blacklist(
            &state.pool,
            &eve_gateway_api_client()?,
            config.project_group_id,
        )
        .await?
        .into_iter()
        .map(|x| x.type_id)
        .collect::<Vec<_>>();

    let blueprint_overwrites = list_default_blueprint_overwrite(
            &state.pool,
            &eve_gateway_api_client()?,
            config.project_group_id,
        )
        .await?
        .into_iter()
        .map(|x| (x.item.type_id, BlueprintBonus { material: x.material_efficiency as f32, ptype_id: x.item.type_id, time: 0f32}))
        .collect::<HashMap<_, _>>();

    let mapping = industry_hubs
        .structures
        .iter()
        .map(|x| {
            let categories = x
                .rigs
                .iter()
                .flat_map(|y| y.categories.clone())
                .map(|y| *y.category_id)
                .collect::<Vec<_>>();
            let groups = x
                .rigs
                .iter()
                .flat_map(|y| y.groups.clone())
                .map(|y| *y.group_id)
                .collect::<Vec<_>>();
            let mut category_groups = Vec::new();
            category_groups.extend(categories);
            category_groups.extend(groups);

            StructureMapping {
                category_group: category_groups,
                structure_uuid: x.id,
            }
        })
        .collect::<Vec<_>>();

    let mut system_index = HashMap::new();
    let index = &eve_gateway_api_client()?
        .fetch_system_index(
            industry_hubs.structures.get(0).unwrap().system.system_id,
        )
        .await
        .unwrap()
        .unwrap();
    system_index.insert(
        industry_hubs.structures.get(0).unwrap().system.system_id,
        (
            index.manufacturing,
            index.reaction,
        )
    );

    let market_prices = market_api_client()?
        .all_prices()
        .await?
        .into_iter()
        .map(|x| (x.type_id, x.adjusted_price))
        .collect::<HashMap<_, _>>();

    let project_config = ProjectConfigBuilder::default()
        .add_blacklists(blacklist)
        .add_blueprint_overwrites(blueprint_overwrites)
        .add_structures(industry_hubs.structures)
        .add_structure_mappings(mapping)
        .set_max_runs(splitting)
        .set_material_cost(market_prices.clone())
        .set_system_index(system_index.clone())
        .build();

    let mut dependency_tree = CalculationEngine::new(project_config);

    let eve_gateway_client = eve_gateway_api_client()?;
    for type_id in config.type_ids.iter() {
        let dependency = if let Ok(Some(x)) = eve_gateway_client.fetch_blueprint_json(*type_id).await {
            x.data
        } else {
            continue
        };

        let json = serde_json::to_value(&dependency).unwrap();

        // FIXME: actual quantity
        //if let Ok(x) = Dependency::try_from(dependency.quantity, json) {
        if let Ok(x) = Dependency::try_from(1, json) {
            dependency_tree.add(x);
        } else {
            continue;
        };
    }

    let dependency_result = dependency_tree
        .write_debug_file_named("aaa.json")
        .apply_bonus()
        //.add_stocks(&stock_filtered)
        .finalize();

    let total_cost = dependency_result.total_cost();
    dependency_result.write_debug_file();

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

    let response = TmpResponse {
        material,
        manufacturing,
    };

    Ok(
        (
            StatusCode::OK,
            Json(response)
        )
        .into_response()
    )
}

#[derive(Debug, Deserialize)]
pub struct TmpRequest {
    project_group_id: ProjectGroupUuid,
    type_ids:         Vec<TypeId>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TmpResponse {
    pub material:       Vec<TmpMaterialResponse>,
    pub manufacturing:  Vec<TmpManufacturingResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TmpMaterialResponse {
    pub item:   Item,
    pub needed: f32,
    pub stock:  i32,
    // TODO: add market
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TmpManufacturingResponse {
    pub item:       Item,
    pub runs:       Vec<u32>,
    pub structure:  Option<Structure>,
    pub build_tax:  f32,
    pub time:       f32,
}
