mod multibuy;

use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClientItem;
use starfoundry_lib_market::{BuyStrategy, MarketBulkRequest, MarketBulkResponse};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;

use crate::lp::MarketProblem;
use crate::market::error::Result;
use crate::eve_gateway_api_client;

pub async fn bulk(
    pool:    &PgPool,
    request: MarketBulkRequest,
) -> Result<Vec<MarketBulkResponse>> {
    let mut market_data: HashMap<TypeId, Vec<MarketEntry>> = HashMap::new();
    let market_items = if let Some(items) = request.item_list {
        items
    } else {
        return Ok(Vec::new());
    };

    let mut type_ids = market_items
        .iter()
        .map(|x| x.type_id)
        .collect::<Vec<_>>();
    type_ids.push(62402.into()); // Compressed Fullerite-C28

    let items = eve_gateway_api_client()
        .unwrap()
        .fetch_item_bulk(type_ids.clone())
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let market_entries = sqlx::query!("
            SELECT *
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
        .map(|x| MarketEntry {
            order_id:     x.order_id,
            structure_id: x.structure_id.into(),
            price:        x.price,
            quantity:     x.remaining,
            item_volume:  items.get(&x.type_id.into()).unwrap().volume as f64,
            type_id:      x.type_id.into(),
        })
        .collect::<Vec<_>>();

    if request.strategy == BuyStrategy::SmartBuy {
        // TODO: add compression
        let start = std::time::Instant::now();
        let mut results: Vec<MarketBulkResponse> = Vec::new();

        market_entries
            .into_iter()
            .for_each(|x| {
                market_data
                    .entry(x.type_id.into())
                    .and_modify(|y: &mut Vec<MarketEntry>| y.push(x.clone()))
                    .or_insert(vec![x]);
            });

        for item in market_items {
            if !market_data.contains_key(&item.type_id) {
                continue;
            }

            let mut data = market_data.get(&item.type_id).unwrap().clone();

            if item.type_id == TypeId(30375) {
                data.extend(market_data.get(&TypeId(62402)).unwrap().clone());
                data.sort_by(|a, b| a.price.total_cmp(&b.price));
            }

            let mut lp = MarketProblem::new();
            lp.calculate_market(data.clone());
            let result = lp.solve(item.quantity);

            let result = result
                .into_iter()
                .map(|(structure_id, x)| MarketBulkResponse {
                    insufficient_data: false,
                    price: x.price,
                    quantity: x.quantity as u64,
                    source: structure_id.into(),
                    type_id: *x.type_id,
                })
                .collect::<Vec<_>>();
            results.extend(result);
        }
        dbg!("smart", start.elapsed().as_millis());
        return Ok(results);
    } else if request.strategy == BuyStrategy::MultiBuy {
        let entries = self::multibuy::multibuy(
            market_items,
            market_entries,
        );
        return Ok(entries);
    } else {
        return Ok(Vec::new());
    }
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
