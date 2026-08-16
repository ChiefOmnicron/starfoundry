use chrono::{NaiveDateTime, Utc};
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_market::{MarketBulkResponse, MarketItem, MarketPrice};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;

use crate::market::service::MarketEntry;

pub fn appraisal(
    items:          HashMap<TypeId, Item>,
    wanted_items:   Vec<MarketItem>,
    market_entries: Vec<MarketEntry>,
    last_fetched:   HashMap<StructureId, NaiveDateTime>,
) -> Vec<MarketBulkResponse> {
    let mut solutions = Vec::new();

    for item in wanted_items {
        let item_info = if let Some(x) = items.get(&item.type_id) {
            x.clone()
        } else {
            continue;
        };

        let structure_id = if let Some(x) = market_entries.first() {
            x.structure_id
        } else {
            return Vec::new();
        };

        // market_entries are already sorted by price
        let buy_prices = market_entries
            .iter()
            .filter(|x| x.type_id == item.type_id)
            .filter(|x| x.is_buy)
            .collect::<Vec<_>>();
        let highest_buy = buy_prices.last().map(|x| x.price).unwrap_or(0f64);
        let lowest_buy = buy_prices.first().map(|x| x.price).unwrap_or(0f64);
        let order_buy = buy_prices.iter().map(|x| x.quantity).sum();

        // market_entries are already sorted by price
        let sell_prices = market_entries
            .iter()
            .filter(|x| x.type_id == item.type_id)
            .filter(|x| !x.is_buy)
            .collect::<Vec<_>>();
        let highest_sell = sell_prices.last().map(|x| x.price).unwrap_or(0f64);
        let lowest_sell = sell_prices.first().map(|x| x.price).unwrap_or(0f64);
        let order_sell = sell_prices.iter().map(|x| x.quantity).sum();

        let insufficient_data = if highest_buy > 0f64 && lowest_sell > 0f64 {
            false
        } else {
            true
        };

        let last_fetch = if let Some(x) = last_fetched.get(&structure_id) {
            x
        } else {
            &Utc::now().naive_utc()
        };

        solutions.push(MarketBulkResponse {
            source:             structure_id,
            item:               item_info,
            quantity:           item.quantity as u64,
            price:              0f64,
            insufficient_data:  insufficient_data,
            buy_price:          Some(MarketPrice {
                                    max: highest_buy,
                                    min: lowest_buy,
                                    total_orders: order_buy,
                                }),
            sell_price:         Some(MarketPrice {
                                    max: highest_sell,
                                    min: lowest_sell,
                                    total_orders: order_sell,
                                }),
            last_fetch:         Some(last_fetch.clone()),
        });
    }

    solutions
}

#[cfg(test)]
mod bulk_appraisal_tests {
    use starfoundry_lib_eve_gateway::{Category, Group, Item};
    use starfoundry_lib_market::MarketItem;
    use starfoundry_lib_types::{StructureId, TypeId};
    use std::collections::HashMap;

    use crate::market::MarketEntry;

    fn items() -> HashMap<TypeId, Item> {
        let mut items = HashMap::new();
        items.insert(1.into(), Item {
            category: Category {
                category_id: 1.into(),
                name: "Test Category".into(),
            },
            group: Group {
                category_id: 1.into(),
                group_id: 1.into(),
                name: "Test Group".into(),
            },
            name: "Test item".into(),
            type_id: 1.into(),
            volume: 0f32,
            meta_group: None,
            repackaged: None,
        });
        items
    }

    #[test]
    fn happy_path() {
        let market_entries = vec![
            MarketEntry {
                item_volume: 0f64,
                order_id: 0i64,
                price: 1f64,
                quantity: 100i32,
                structure_id: 1i64.into(),
                type_id: TypeId(1),
                is_buy: false,
            },
            MarketEntry {
                item_volume: 0f64,
                order_id: 0i64,
                price: 3f64,
                quantity: 100i32,
                structure_id: 1i64.into(),
                type_id: TypeId(1),
                is_buy: false,
            },
            MarketEntry {
                item_volume: 0f64,
                order_id: 1i64,
                price: 2f64,
                quantity: 500i32,
                structure_id: 2i64.into(),
                type_id: TypeId(1),
                is_buy: true,
            },
            MarketEntry {
                item_volume: 0f64,
                order_id: 1i64,
                price: 4f64,
                quantity: 500i32,
                structure_id: 2i64.into(),
                type_id: TypeId(1),
                is_buy: true,
            },
        ];

        let wanted = vec![
            MarketItem {
                quantity: 5,
                type_id:  TypeId(1),
            }
        ];

        let result = super::appraisal(items(), wanted, market_entries, HashMap::new());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].price, 0f64);
        assert_eq!(result[0].buy_price.clone().unwrap().max, 4f64);
        assert_eq!(result[0].buy_price.clone().unwrap().min, 2f64);
        assert_eq!(result[0].sell_price.clone().unwrap().max, 3f64);
        assert_eq!(result[0].sell_price.clone().unwrap().min, 1f64);
        assert_eq!(result[0].quantity, 5);
        assert_eq!(result[0].insufficient_data, false);
        assert_eq!(result[0].source, StructureId(1));
    }
}
