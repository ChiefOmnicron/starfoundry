mod multibuy;
mod smartbuy;

use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClientItem;
use starfoundry_lib_market::{Asteroid, BuyStrategy, Gas, MarketBulkRequest, MarketBulkResponse};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;

use crate::market::error::Result;
use crate::eve_gateway_api_client;
use crate::market::last_fetched;

pub async fn bulk(
    pool:    &PgPool,
    request: MarketBulkRequest,
) -> Result<Vec<MarketBulkResponse>> {
    let market_items = if let Some(items) = request.item_list {
        items
    } else {
        return Ok(Vec::new());
    };

    let mut type_ids = market_items
        .iter()
        .map(|x| x.type_id)
        .collect::<Vec<_>>();

    // FIXME: only add them if compression is active
    // TODO: make them configurable
    type_ids.extend(Asteroid::compressed_asteroid_type_ids());
    type_ids.extend(Asteroid::compressed_moon_type_ids());
    type_ids.extend(Gas::compressed_type_ids());

    let items = eve_gateway_api_client()?
        .fetch_item_bulk(type_ids.clone())
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let market_entries = sqlx::query!("
            SELECT
                order_id,
                structure_id,
                price,
                remaining,
                virtual_remaining,
                type_id
            FROM market_order_latest mol
            WHERE mol.type_id = ANY($1)
            AND mol.structure_id = ANY($2)
            AND mol.is_buy = false
            ORDER BY mol.price ASC
        ",
            &type_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
            &request.markets.iter().map(|x| **x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| {
            let quantity = if request.virtual_market {
                x.virtual_remaining
            } else {
                x.remaining
            };

            MarketEntry {
                order_id:     x.order_id,
                structure_id: x.structure_id.into(),
                price:        x.price,
                quantity:     quantity,
                item_volume:  items.get(&x.type_id.into()).unwrap().volume as f64,
                type_id:      x.type_id.into(),
            }
        })
        .collect::<Vec<_>>();

    let mut market_last_fetch = HashMap::new();
    let mut structure_ids = market_entries
        .iter()
        .map(|x| x.structure_id)
        .collect::<Vec<_>>();
    structure_ids.sort();
    structure_ids.dedup();

    for structure_id in request.markets.iter() {
        if market_last_fetch.contains_key(structure_id) {
            continue;
        }

        if let Ok(x) = last_fetched(
                pool,
                structure_id,
            )
            .await {

            market_last_fetch.insert(*structure_id, x);
        } else {
            continue;
        }
    }

    let result = if request.strategy == BuyStrategy::SmartBuy {
        let config = if let Some(x) = request.smart_buy_config {
            x
        } else {
            return Ok(Vec::new());
        };

        self::smartbuy::smartbuy(
            market_items,
            market_entries,
            market_last_fetch,
            config,
        )
    } else if request.strategy == BuyStrategy::MultiBuy {
        self::multibuy::multibuy(
            market_items,
            market_entries,
            market_last_fetch,
        )
    } else {
        Vec::new()
    };
    Ok(result)
}

#[derive(Clone, Debug)]
pub struct MarketEntry {
    pub order_id:     i64,
    pub structure_id: StructureId,
    pub price:        f64,
    pub quantity:     i32,
    pub item_volume:  f64,
    pub type_id:      TypeId,
}
