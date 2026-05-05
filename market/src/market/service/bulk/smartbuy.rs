use chrono::NaiveDateTime;
use starfoundry_lib_market::{Asteroid, Gas, MarketBulkResponse, MarketItemList, Mineral, SmartBuyConfig};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;

use crate::lp::{AsteroidCompressionProblem, MarketProblem};
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

        let minerals = market_items
            .iter()
            .filter(|x| Asteroid::mineral_type_ids().contains(&x.type_id))
            .map(|x| (Mineral::from(x.type_id), x.quantity as f64))
            .collect::<HashMap<_, _>>();

        let mut lp = AsteroidCompressionProblem::new(
            config.mineral_compression,
        );
        lp.define_problem(market_entries);
        let result = lp.solve(minerals.clone());

        if let Ok(x) = result {
            let result = x.into_iter()
                .map(|((structure_id, _), x)| MarketBulkResponse {
                    insufficient_data:  false,
                    price:              x.price,
                    quantity:           x.quantity as u64,
                    source:             structure_id,
                    type_id:            x.type_id,
                    last_fetch:         last_fetched.get(&structure_id.into()).cloned(),
                })
                .collect::<Vec<_>>();
            results.extend(result);
        } else {
            for mineral_type_id in Asteroid::mineral_type_ids() {
                let quantity = minerals
                    .get(&Mineral::from(mineral_type_id))
                    .unwrap_or(&0f64);

                results.push(MarketBulkResponse {
                    insufficient_data:  true,
                    price:              0f64,
                    quantity:           *quantity as u64,
                    source:             StructureId(0),
                    type_id:            mineral_type_id,
                    last_fetch:         None,
                });
            }
        }
    }

    for item in market_items.iter() {
        if !market_data.contains_key(&item.type_id) {
            continue;
        }

        if config.mineral_compression.is_some() &&
            Asteroid::mineral_type_ids().contains(&item.type_id) {

            continue;
        }

        let mut data = market_data.get(&item.type_id).unwrap().clone();
        if config.gas_decompression.is_some() {
            if let Ok(gas) = Gas::try_from(item.type_id) {
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
        let result = if Gas::is_gas(item.type_id) && let Some(x) = config.gas_decompression {
            lp.solve(x.decompressed_quantity(item.quantity))
        } else {
            lp.solve(item.quantity)
        };

        if let Ok(x) = result {
            let result = x.into_iter()
                .map(|(structure_id, x)| MarketBulkResponse {
                    insufficient_data:  false,
                    price:              x.price,
                    quantity:           x.quantity as u64,
                    source:             structure_id.into(),
                    type_id:            x.type_id,
                    last_fetch:         last_fetched.get(&structure_id.into()).cloned(),
                })
                .collect::<Vec<_>>();
            results.extend(result);
        } else {
            results.push(MarketBulkResponse {
                insufficient_data:  true,
                price:              0f64,
                quantity:           item.quantity as u64,
                source:             StructureId(0),
                type_id:            item.type_id,
                last_fetch:         None,
            });
        }

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
