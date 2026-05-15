use starfoundry_lib_market::{MarketBulkResponse, MarketItemList};
use starfoundry_lib_types::{StructureId, TypeId};
use std::collections::HashMap;

use crate::market::service::MarketEntry;
use chrono::NaiveDateTime;

pub fn appraisal(
    wanted_items:   Vec<MarketItemList>,
    market_entries: Vec<MarketEntry>,
    last_fetched:   HashMap<StructureId, NaiveDateTime>,
) -> Vec<MarketBulkResponse> {
    let mut solutions = Vec::new();

    for item in wanted_items {
        let mut result = CalculationHelper {
            buy_price:          0f64,
            sell_price:         0f64,
            quantity:           0u64,
            insufficient_data:  true,
            source:             0i64,
            type_id:            *item.type_id,
        };

        // market_entries are already sorted by price
        let highest_buy = market_entries
            .iter()
            .filter(|x| x.type_id == item.type_id)
            .filter(|x| x.is_buy)
            .collect::<Vec<_>>();
        let highest_buy = highest_buy.last();
        if let Some(x) = highest_buy {
            result.buy_price = x.price;
        }

        // market_entries are already sorted by price
        let lowest_sell = market_entries
            .iter()
            .filter(|x| x.type_id == item.type_id)
            .filter(|x| !x.is_buy)
            .collect::<Vec<_>>();
        let lowest_sell = lowest_sell.first();
        if let Some(x) = lowest_sell {
            result.sell_price = x.price;
        }

        if result.buy_price > 0f64 && result.sell_price > 0f64 {
            result.insufficient_data = false;
        }
    }

    solutions
}

#[derive(Clone, Debug, Default)]
struct CalculationHelper {
    pub source:             i64,
    pub type_id:            i32,
    pub quantity:           u64,
    pub buy_price:          f64,
    pub sell_price:         f64,
    pub insufficient_data:  bool,
}

#[cfg(test)]
mod bulk_multibuy_tests {
    use starfoundry_lib_types::{StructureId, TypeId};
    use starfoundry_lib_market::MarketItemList;
    
    use crate::market::MarketEntry;
    use std::collections::HashMap;

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

        let result = super::multibuy(wanted, market_entries, HashMap::new());
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

        let result = super::multibuy(wanted, market_entries, HashMap::new());
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

        let result = super::multibuy(wanted, market_entries, HashMap::new());
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].price, 2f64);
        assert_eq!(result[0].quantity, 3);
        assert_eq!(result[0].insufficient_data, true);
        assert_eq!(result[0].source, StructureId(2));
    }
}
