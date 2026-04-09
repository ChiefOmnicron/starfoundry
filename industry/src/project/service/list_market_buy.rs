use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClientItem, Item};
use starfoundry_lib_market::{BuyStrategy, Gas, MarketApiClientOrder, MarketBulkRequest, MarketBulkResponse, MarketItemList, SmartBuyConfig};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use crate::{market_api_client, sort_by_market_group_flat};
use crate::project::list_market_buy::ListMarketBuyQuery;
use starfoundry_lib_types::TypeId;

pub async fn list_market_buy(
    pool:                   &PgPool,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClientItem,
    config:                 ListMarketBuyQuery,
) -> Result<Vec<ProjectMarketBuy>> {
    let entries = sqlx::query!(r#"
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
    type_ids.extend(Gas::compressed_type_ids());

    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let market_entries = if config.strategy == BuyStrategy::MultiBuy {
        market_api_client()
            .unwrap()
            .bulk_latest_orders(MarketBulkRequest {
                item_list: Some(
                    entries.iter().map(|x| MarketItemList {
                        quantity: x.quantity,
                        type_id: x.type_id.into(),
                    })
                    .collect::<Vec<_>>()
                ),
                markets: vec![1046664001931i64, 1049588174021i64, 60003760i64, 60008494i64].into_iter().map(Into::into).collect::<Vec<_>>(),
                //markets: vec![1046664001931i64, 60003760i64, 60008494i64].into_iter().map(Into::into).collect::<Vec<_>>(),
                strategy: BuyStrategy::MultiBuy,
                ..Default::default()
            })
            .await
            .unwrap()
    } else if config.strategy == BuyStrategy::SmartBuy {
        market_api_client()
            .unwrap()
            .bulk_latest_orders(MarketBulkRequest {
                item_list: Some(
                    entries.iter().map(|x| MarketItemList {
                        quantity: x.quantity,
                        type_id: x.type_id.into(),
                    })
                    .collect::<Vec<_>>()
                ),
                markets: vec![1046664001931i64, 1049588174021i64, 60003760i64, 60008494i64].into_iter().map(Into::into).collect::<Vec<_>>(),
                //markets: vec![1046664001931i64, 60003760i64, 60008494i64].into_iter().map(Into::into).collect::<Vec<_>>(),
                strategy: BuyStrategy::SmartBuy,
                smart_buy_config: Some(SmartBuyConfig {
                    gas_compression: true,
                    ..Default::default()
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
        let item = if let Some(x) = items.get(&entry.type_id.into()) {
            x
        } else {
            continue;
        };

        let market_entry = market_entries
            .iter()
            .cloned()
            .filter(|x| x.type_id == entry.type_id)
            .collect::<Vec<_>>();

        let project_group = ProjectMarketBuy {
            id:         entry.id.into(),
            item:       item.clone(),
            quantity:   entry.quantity,

            cost:       entry.cost,
            source:     entry.source.clone(),

            entries:    market_entry,
        };
        project_market.push(project_group);
    }

    for compressed_gas in Gas::compressed_type_ids() {
        let entry = if let Some(x) = entries
            .iter()
            .find(|x| TypeId(x.type_id) == Gas::from(compressed_gas).to_uncompressed_type_id()) {
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
            .cloned()
            .filter(|x| x.type_id == *compressed_gas)
            .collect::<Vec<_>>();

        let project_group = ProjectMarketBuy {
            id:         entry.id.into(),
            item:       item.clone(),
            quantity:   entry.quantity,

            cost:       entry.cost,
            source:     entry.source.clone(),

            entries:    market_entry,
        };
        project_market.push(project_group);
    }

    Ok(sort_market(project_market))
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectMarketBuy {
    pub id:       Uuid,
    pub item:     Item,
    pub quantity: i32,

    pub cost:     Option<f64>,
    pub source:   Option<String>,

    pub entries:  Vec<MarketBulkResponse>,
}

sort_by_market_group_flat!(sort_market, ProjectMarketBuy);
