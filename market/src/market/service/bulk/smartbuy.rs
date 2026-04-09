use chrono::NaiveDateTime;
use starfoundry_lib_market::{Gas, MarketBulkResponse, MarketItemList, SmartBuyConfig};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;

use crate::lp::MarketProblem;
use crate::market::MarketEntry;

pub fn smartbuy(
    market_items:   Vec<MarketItemList>,
    market_entries: Vec<MarketEntry>,
    last_fetched:   HashMap<StructureId, NaiveDateTime>,
    config:         SmartBuyConfig,
) -> Vec<MarketBulkResponse> {
    let mut market_data: HashMap<TypeId, Vec<MarketEntry>> = HashMap::new();

    // TODO: add compression
    let start = std::time::Instant::now();
    let mut results: Vec<MarketBulkResponse> = Vec::new();

    market_entries
        .iter()
        .for_each(|x| {
            market_data
                .entry(x.type_id.into())
                .and_modify(|y: &mut Vec<MarketEntry>| y.push(x.clone()))
                .or_insert(vec![x.clone()]);
        });

    for item in market_items.iter() {
        if !market_data.contains_key(&item.type_id) {
            continue;
        }

        let mut data = market_data.get(&item.type_id).unwrap().clone();
        if config.gas_compression {
            if Gas::is_gas(item.type_id) {
                let gas = Gas::from(item.type_id);
                if gas.is_uncompressed() {
                    let market_data = market_data
                        .get(&gas.to_compressed_type_id())
                        .map(Clone::clone)
                        .unwrap_or_default()
                        .clone();
                    data.extend(market_data);
                    data.sort_by(|a, b| a.price.total_cmp(&b.price));
                }
            }
        }

        let mut lp = MarketProblem::new();
        lp.calculate_market(data.clone());

        // increase required amount
        let result = if Gas::is_gas(item.type_id) && config.gas_compression {
            lp.solve(compression_quantity_modifier(item.quantity as f64, 95f64))
        } else {
            lp.solve(item.quantity)
        };

        let result = result
            .into_iter()
            .map(|(structure_id, x)| MarketBulkResponse {
                insufficient_data:  false,
                price:              x.price,
                quantity:           x.quantity as u64,
                source:             structure_id.into(),
                type_id:            *x.type_id,
                last_fetch:         last_fetched.get(&structure_id.into()).cloned(),
            })
            .collect::<Vec<_>>();
        results.extend(result);
    }
    dbg!("smart", start.elapsed().as_millis());
    results
}

fn compression_quantity_modifier(
    quantity:                 f64,
    // for example 95f64
    decompression_efficiency: f64
) -> i32 {
    let mut percent_increase = decompression_efficiency - 100f64;
    if percent_increase.is_sign_negative() {
        percent_increase *= -1f64;
    }
    percent_increase /= decompression_efficiency;
    percent_increase += 1f64;

    (quantity * percent_increase).ceil() as i32
}
