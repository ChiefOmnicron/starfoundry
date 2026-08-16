use chrono::NaiveDateTime;
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_market::{Asteroid, Gas, MarketBulkResponse, MarketItem, Mineral, SmartBuyConfig};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;

use crate::lp::{AsteroidCompressionProblem, MarketProblem};
use crate::market::MarketEntry;

pub fn smartbuy(
    items:          HashMap<TypeId, Item>,
    wanted_items:   Vec<MarketItem>,
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

    // check if mineral compression is active and search for those first
    if config.mineral_compression.is_some() {
        let market_entries = market_data
            .iter()
            .filter(|(type_id, _)|
                // TODO: make them configurable
                Asteroid::mineral_type_ids().contains(&type_id) ||
                Asteroid::asteroid_type_ids().contains(&type_id) ||
                Asteroid::compressed_asteroid_type_ids().contains(&type_id) ||
                Asteroid::compressed_moon_type_ids().contains(&type_id)
            )
            .flat_map(|(_, x)| x)
            .cloned()
            .collect::<Vec<_>>();

        let minerals = wanted_items
            .iter()
            .filter(|x| Asteroid::mineral_type_ids().contains(&x.type_id))
            .map(|x| (Mineral::from(x.type_id), x.quantity as f64))
            .collect::<HashMap<_, _>>();

        let mut lp = AsteroidCompressionProblem::new(
            config.mineral_compression,
        );
        lp.define_problem(market_entries.clone());
        let result = lp.solve(minerals.clone());

        if let Ok(x) = result {
            for ((structure_id, type_id), market_result) in x.into_iter() {
                let item = if let Some(x) = items.get(&type_id) {
                    x
                } else {
                    continue;
                };

                results.push(MarketBulkResponse {
                    insufficient_data:  false,
                    price:              market_result.price,
                    buy_price:          None,
                    sell_price:         None,
                    quantity:           market_result.quantity as u64,
                    source:             structure_id,
                    item:               item.clone(),
                    last_fetch:         last_fetched.get(&structure_id).cloned(),
                });
            }
        } else {
            for mineral_type_id in Asteroid::mineral_type_ids() {
                let quantity = minerals
                    .get(&Mineral::from(mineral_type_id))
                    .unwrap_or(&0f64);

                let item = if let Some(x) = items.get(&mineral_type_id) {
                    x
                } else {
                    continue;
                };

                results.push(MarketBulkResponse {
                    insufficient_data:  true,
                    price:              0f64,
                    buy_price:          None,
                    sell_price:         None,
                    quantity:           *quantity as u64,
                    source:             StructureId(0),
                    item:               item.clone(),
                    last_fetch:         None,
                });
            }
        }
    }

    // go through all items and find the best matching prices
    for wanted_item in wanted_items.iter() {
        if !market_data.contains_key(&wanted_item.type_id) {
            continue;
        }

        if config.mineral_compression.is_some() &&
            Asteroid::mineral_type_ids().contains(&wanted_item.type_id) {

            continue;
        }

        let mut data = market_data.get(&wanted_item.type_id).unwrap().clone();
        if config.gas_decompression.is_some() {
            if let Ok(gas) = Gas::try_from(wanted_item.type_id) {
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
        let result = if Gas::is_gas(wanted_item.type_id) && let Some(x) = config.gas_decompression {
            lp.solve(x.compression_quantity(wanted_item.quantity))
        } else {
            lp.solve(wanted_item.quantity)
        };

        let item = if let Some(x) = items.get(&wanted_item.type_id) {
            x
        } else {
            continue;
        };

        if let Ok(x) = result {
            let result = x.into_iter()
                .map(|(structure_id, x)| MarketBulkResponse {
                    insufficient_data:  false,
                    price:              x.price,
                    buy_price:          None,
                    sell_price:         None,
                    quantity:           x.quantity as u64,
                    source:             structure_id.into(),
                    item:               item.clone(),
                    last_fetch:         last_fetched.get(&structure_id.into()).cloned(),
                })
                .collect::<Vec<_>>();
            results.extend(result);
        } else {
            results.push(MarketBulkResponse {
                insufficient_data:  true,
                price:              0f64,
                buy_price:          None,
                sell_price:         None,
                quantity:           wanted_item.quantity as u64,
                source:             StructureId(0),
                item:               item.clone(),
                last_fetch:         None,
            });
        }

    }
    dbg!("smart", start.elapsed().as_millis());
    results
}
