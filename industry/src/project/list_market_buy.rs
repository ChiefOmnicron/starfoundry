use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_eve_gateway::item::{
    Asteroid, EveGatewayApiClientItem, Gas, GasDecompressionEfficiency, OreReprocessingEfficiency
};
use starfoundry_lib_types::{StructureId, TypeId};
use starfoundry_market::market::{
    MarketApiClientOrder, MarketBulkRequest, MarketItem, MarketStrategy, SmartBuyConfig
};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::api_docs::{
    BadRequest, InternalServerError, NotFound, Unauthorized
};
use crate::project::error::{ProjectError, Result};
use crate::project::fetch::ProjectMarketBuy;
use crate::{
    AppState, ProjectUuid, eve_gateway_api_client, market_api_client, sort_by_market_group_flat
};

/// List Market
///
/// - Alternative route: `/latest/projects/{ProjectUuid}/market/buy`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market/buy`
///
/// ---
///
/// Lists all materials that need to be bought
///
/// ## Security
/// - authenticated
/// - project:read
#[utoipa::path(
    post,
    path = "/{ProjectUuid}/market/buy",
    tag = "projects",
    params(
        ProjectUuid,
        ListMarketBuyQuery,
    ),
    responses(
        (
            body = Vec<ProjectMarketBuy>,
            description = "List all materials for a project",
            status = OK,
        ),
        (
            description = "There aren't any materials required for the project",
            status = NO_CONTENT,
        ),
        NotFound,
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(project_id): Path<ProjectUuid>,
    Json(config): Json<ListMarketBuyQuery>,
) -> Result<impl IntoResponse> {
    let data = list_market_buy(
        &state.postgres,
        project_id,
        &eve_gateway_api_client()?,
        config,
    )
    .await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ListMarketBuyQuery {
    pub strategy:      MarketStrategy,
    pub structure_ids: Vec<StructureId>,

    // gas decompression is active
    pub gas_decompression:   Option<GasDecompressionEfficiency>,
    // mineral compression is active
    pub mineral_compression: Option<OreReprocessingEfficiency>,
}

pub async fn list_market_buy(
    pool: &PgPool,
    project_id: ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClientItem,
    config: ListMarketBuyQuery,
) -> Result<Vec<ProjectMarketBuy>> {
    let entries = sqlx::query!(
        r#"
            SELECT
                id,
                cost,
                type_id,
                quantity,
                source
            FROM project_market
            WHERE project_id = $1
            AND cost IS NULL
        "#,
        *project_id,
    )
    .fetch_all(pool)
    .await
    .map_err(ProjectError::ListMisc)?;

    let mut type_ids = entries
        .iter()
        .map(|x| x.type_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    // TODO: better solution
    // TODO: make them configurable
    type_ids.extend(Asteroid::compressed_asteroid_type_ids());
    type_ids.extend(Asteroid::compressed_moon_type_ids());
    type_ids.extend(Gas::compressed_type_ids());

    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let market_entries = if config.strategy == MarketStrategy::MultiBuy {
        market_api_client()
            .unwrap()
            .bulk_latest_orders(MarketBulkRequest {
                item_list: Some(
                    entries
                        .iter()
                        .map(|x| {
                            MarketItem {
                                quantity: x.quantity,
                                type_id:  x
                                    .type_id
                                    .into(),
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                markets: config.structure_ids,
                strategy: MarketStrategy::MultiBuy,
                virtual_market: true,
                ..Default::default()
            })
            .await
            .unwrap()
    } else if config.strategy == MarketStrategy::SmartBuy {
        market_api_client()
            .unwrap()
            .bulk_latest_orders(MarketBulkRequest {
                item_list: Some(
                    entries
                        .iter()
                        .map(|x| {
                            MarketItem {
                                quantity: x.quantity,
                                type_id:  x
                                    .type_id
                                    .into(),
                            }
                        })
                        .collect::<Vec<_>>(),
                ),
                markets: config.structure_ids,
                strategy: MarketStrategy::SmartBuy,
                virtual_market: true,
                smart_buy_config: Some(SmartBuyConfig {
                    gas_decompression:   config.gas_decompression,
                    mineral_compression: config.mineral_compression,
                }),
                ..Default::default()
            })
            .await
            .unwrap()
    } else {
        Vec::new()
    };

    let mut project_market = Vec::new();
    for entry in entries.iter() {
        let item = if let Some(x) = items.get(
            &entry
                .type_id
                .into(),
        ) {
            x
        } else {
            continue;
        };

        let market_entry = market_entries
            .iter()
            .filter(|x| {
                x.item.type_id
                    == entry
                        .type_id
                        .into()
            })
            .cloned()
            .collect::<Vec<_>>();

        let project_group = ProjectMarketBuy {
            id:       entry.id,
            item:     item.clone(),
            quantity: entry.quantity,

            cost:   entry.cost,
            source: entry
                .source
                .clone(),

            entries: market_entry,
        };
        project_market.push(project_group);
    }

    for compressed_gas in Gas::compressed_type_ids() {
        let entry = if let Some(x) = entries
            .iter()
            .find(|x| {
                TypeId(x.type_id)
                    == Gas::try_from(compressed_gas)
                        .unwrap()
                        .to_uncompressed_type_id()
            }) {
            x
        } else {
            continue;
        };

        let item = if let Some(x) = items.get(&compressed_gas) {
            x
        } else {
            continue;
        };

        let market_entry = market_entries
            .iter()
            .filter(|x| x.item.type_id == compressed_gas)
            .cloned()
            .collect::<Vec<_>>();

        let project_group = ProjectMarketBuy {
            id:       entry.id,
            item:     item.clone(),
            quantity: entry.quantity,

            cost:   entry.cost,
            source: entry
                .source
                .clone(),

            entries: market_entry,
        };
        project_market.push(project_group);
    }

    // TODO: make them configurable
    let asteroid_type_ids = vec![
        Asteroid::compressed_asteroid_type_ids(),
        Asteroid::compressed_moon_type_ids(),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>();

    for type_id in asteroid_type_ids {
        let item = if let Some(x) = items.get(&type_id) {
            x
        } else {
            continue;
        };

        let market_entry = market_entries
            .iter()
            .filter(|x| x.item.type_id == type_id)
            .cloned()
            .collect::<Vec<_>>();

        let project_group = ProjectMarketBuy {
            // UUID type doesn't matter
            id:       Uuid::now_v7(),
            item:     item.clone(),
            quantity: 0,

            cost:   None,
            source: None,

            entries: market_entry,
        };
        project_market.push(project_group);
    }

    Ok(sort_market(project_market))
}

sort_by_market_group_flat!(sort_market, ProjectMarketBuy);
