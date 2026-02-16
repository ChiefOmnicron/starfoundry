use starfoundry_lib_market::{MarketBulkResponse, MarketItemList};
use starfoundry_lib_types::TypeId;
use std::collections::HashMap;

use crate::market::service::MarketEntry;

pub fn multibuy(
    wanted_items:   Vec<MarketItemList>,
    market_entries: Vec<MarketEntry>,
) -> Vec<MarketBulkResponse> {
    let mut viable_markets: HashMap<TypeId, CalculationHelper> = HashMap::new();

    for item in wanted_items {
        // Group all prices by the station_id and type_id
        let mut grouped_by_station = HashMap::new();
        for price in market_entries.iter().filter(|x| x.type_id == item.type_id) {
            let entry = CalculationHelper {
                insufficient_data: false,
                price: price.price,
                quantity: item.quantity as u64,
                remaining: price.quantity as u64,
                source: *price.structure_id,
                type_id: *item.type_id,
            };

            grouped_by_station
                .entry((price.structure_id, price.type_id))
                .and_modify(|x: &mut Vec<CalculationHelper>| x.push(entry.clone()))
                .or_insert(vec![entry.clone()]);
        }

        // Sort the vectors by price
        for (_, entries) in grouped_by_station.iter_mut() {
            entries.sort_by_key(|x| x.price.floor() as u64);
        }

        // all markets listed are not able to fulfil the request
        //
        // the list is required later, if no markets are found that can fulfil
        // the request
        let mut non_viable_markets = Vec::new();
        for ((_, type_id), entries) in grouped_by_station {
            let mut selected = CalculationHelper::default();

            for entry in entries {
                if selected.quantity == 0 {
                    selected = entry;
                    continue;
                }

                // If there are more remaining entries than the quantity needed,
                // it's a valid market
                if selected.remaining >= selected.quantity {
                    if let Some(x) = viable_markets.get(&type_id) {
                        if x.price > selected.price {
                            viable_markets.insert(type_id, selected.clone());
                        }
                    } else {
                        viable_markets.insert(type_id, selected.clone());
                    }
                }

                selected.remaining += entry.remaining;

                // If the price from the current entry is higher than the old price,
                // set the new value
                if entry.price > selected.price {
                    selected.price = entry.price;
                }
            }

            // The market does not have enough to support the wanted
            if selected.remaining < selected.quantity {
                non_viable_markets.push(selected);
                continue;
            }

            if selected.remaining >= selected.quantity {
                if let Some(x) = viable_markets.get(&type_id) {
                    if selected.price < x.price {
                        viable_markets.insert(type_id, selected.clone());
                    }
                } else {
                    viable_markets.insert(type_id, selected.clone());
                }
                continue;
            }
        }

        // no market had enough materials to fulfil the request
        // take the most expensive one -> prefer an over estimation over an
        // under estimation
        if !viable_markets.contains_key(&item.type_id) {
            let mut solution = CalculationHelper::default();

            for market in non_viable_markets {
                if (market.remaining as f64 * market.price) > (solution.remaining as f64 * solution.price) {
                    solution = market;
                    solution.insufficient_data = true;
                }
            }

            viable_markets.insert(item.type_id, solution);
        }
    }

    viable_markets
        .into_iter()
        .map(|(_, x)| MarketBulkResponse {
            insufficient_data:  x.insufficient_data,
            price:              x.price,
            quantity:           x.quantity,
            source:             x.source.into(),
            type_id:            x.type_id,
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Debug, Default)]
struct CalculationHelper {
    pub source:             i64,
    pub type_id:            i32,
    pub quantity:           u64,
    pub price:              f64,
    pub insufficient_data:  bool,
    pub remaining:          u64,
}

#[cfg(test)]
mod bulk_multibuy_tests {
    use starfoundry_lib_types::{StructureId, TypeId};
    use starfoundry_lib_market::MarketItemList;
    
    use crate::market::MarketEntry;

    #[test]
    fn no_overflow() {
        let market_entries = vec![
            MarketEntry {
                item_volume: 0f64,
                order_id: 0i64,
                price: 1f64,
                quantity: 100i32,
                structure_id: 1i64.into(),
                type_id: TypeId(1),
            },
            MarketEntry {
                item_volume: 0f64,
                order_id: 1i64,
                price: 2f64,
                quantity: 500i32,
                structure_id: 2i64.into(),
                type_id: TypeId(1),
            }
        ];

        let wanted = vec![
            MarketItemList {
                quantity: 5,
                type_id:  TypeId(1),
            }
        ];

        let result = super::multibuy(wanted, market_entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].price, 1f64);
        assert_eq!(result[0].quantity, 5);
        assert_eq!(result[0].insufficient_data, false);
        assert_eq!(result[0].source, StructureId(1));
    }

    #[test]
    fn with_overflow() {
        let market_entries = vec![
            MarketEntry {
                item_volume: 0f64,
                order_id: 0i64,
                price: 1f64,
                quantity: 100i32,
                structure_id: 1i64.into(),
                type_id: TypeId(1),
            },
            MarketEntry {
                item_volume: 0f64,
                order_id: 1i64,
                price: 2f64,
                quantity: 500i32,
                structure_id: 2i64.into(),
                type_id: TypeId(1),
            }
        ];

        let wanted = vec![
            MarketItemList {
                quantity: 101,
                type_id:  TypeId(1),
            }
        ];

        let result = super::multibuy(wanted, market_entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].price, 2f64);
        assert_eq!(result[0].quantity, 101);
        assert_eq!(result[0].insufficient_data, false);
        assert_eq!(result[0].source, StructureId(2));
    }

    #[test]
    fn insufficient_data() {
        let market_entries = vec![
            MarketEntry {
                item_volume: 0f64,
                order_id: 0i64,
                price: 1f64,
                quantity: 1i32,
                structure_id: 1i64.into(),
                type_id: TypeId(1),
            },
            MarketEntry {
                item_volume: 0f64,
                order_id: 1i64,
                price: 2f64,
                quantity: 1i32,
                structure_id: 2i64.into(),
                type_id: TypeId(1),
            }
        ];

        let wanted = vec![
            MarketItemList {
                quantity: 3,
                type_id:  TypeId(1),
            }
        ];

        let result = super::multibuy(wanted, market_entries);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].price, 2f64);
        assert_eq!(result[0].quantity, 3);
        assert_eq!(result[0].insufficient_data, true);
        assert_eq!(result[0].source, StructureId(2));
    }
}
