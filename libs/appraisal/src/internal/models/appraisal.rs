use serde::Serialize;
use sqlx::types::Uuid;
use starfoundry_lib_items::Item;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use super::MarketEntry;

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "code": "cidx8nzQNW",
        "created_at": 1744099132822i64,
        "items": [
            {
                "quantity": 100,
                "type_id": 34,
                "low_data": false,
                "volume": 1.0,
                "meta": {
                    "name": "Tritanium",
                    "volume": 0.01,
                    "category_id": 4,
                    "group_id": 18,
                    "type_id": 34,
                    "meta_group_id": null,
                    "repackaged": null
                },
                "buy": {
                    "max": 409.00001525878906,
                    "min": 0.9999999776482582,
                    "total_orders": 13046927471i64,
                    "per_item": {
                        "avg": 2.9603448458144377,
                        "max": 4.090000152587891,
                        "min": 0.009999999776482582
                    }
                },
                "sell": {
                    "max": 1000000.0,
                    "min": 438.0000114440918,
                    "total_orders": 8652196750i64,
                    "per_item": {
                        "avg": 257.6819049365937,
                        "max": 10000.0,
                        "min": 4.380000114440918
                    }
                }
            },
            {
                "quantity": 100,
                "type_id": 35,
                "low_data": false,
                "volume": 1.0,
                "meta": {
                    "name": "Pyerite",
                    "volume": 0.01,
                    "category_id": 4,
                    "group_id": 18,
                    "type_id": 35,
                    "meta_group_id": null,
                    "repackaged": null
                },
                "buy": {
                    "max": 2100.0,
                    "min": 0.9999999776482582,
                    "total_orders": 3170777127i64,
                    "per_item": {
                        "avg": 14.890000105204601,
                        "max": 21.0,
                        "min": 0.009999999776482582
                    }
                },
                "sell": {
                    "max": 100000000.0,
                    "min": 2317.0000076293945,
                    "total_orders": 1543005951,
                    "per_item": {
                        "avg": 24009.31952381134,
                        "max": 1000000.0,
                        "min": 23.170000076293945
                    }
                }
            }
        ],
        "invalid": [],
        "market_id": 60003760,
        "comment": "",
        "price_modifier": 100,
        "total": {
            "buy": 2509.000015258789,
            "sell": 2755.0000190734863,
            "volume": 2.0
        }
    }),
)]
pub struct Appraisal {
    #[serde(skip)]
    pub id:             Uuid,
    /// code that was used to create the appraisal, will be null if it was for compression or reprocessing
    /// will be 10 characters long, [a-zA-z0-9]
    pub code:           Option<String>,
    /// timestamp when it was created in milliseconds
    pub created_at:     i64,
    /// all parsed items with their pricing information
    pub items:          Vec<AppraisalItem>,
    /// includes all fields that could not be parsed
    pub invalid:        Vec<String>,
    /// market that was used to create the appraisal
    pub market_id:      i64,

    /// initial appraisal input
    pub raw:            Option<String>,

    /// comment for the appraisal
    pub comment:        Option<String>,
    /// the price modifier is applied on the return data
    pub price_modifier: i16,

    pub total:          AppraisalTotal,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "quantity": 100,
        "type_id": 35,
        "low_data": false,
        "volume": 1.0,
        "meta": {
            "name": "Pyerite",
            "volume": 0.01,
            "category_id": 4,
            "group_id": 18,
            "type_id": 35,
            "meta_group_id": null,
            "repackaged": null
        },
        "buy": {
            "max": 2100.0,
            "min": 0.9999999776482582,
            "total_orders": 3170777127i64,
            "per_item": {
                "avg": 14.890000105204601,
                "max": 21.0,
                "min": 0.009999999776482582
            }
        },
        "sell": {
            "max": 100000000.0,
            "min": 2317.0000076293945,
            "total_orders": 1543005951,
            "per_item": {
                "avg": 24009.31952381134,
                "max": 1000000.0,
                "min": 23.170000076293945
            }
        }
    })
)]
pub struct AppraisalItem {
    pub quantity: i64,
    pub type_id:  TypeId,
    /// if there are not enough items on the market to cover the requested amount
    pub low_data: bool,

    pub volume: f32,

    /// general item information
    pub meta: Item,

    /// buy information
    pub buy:  MarketEntry,
    /// sell information
    pub sell: MarketEntry,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "total": {
            "buy": 2509.000015258789,
            "sell": 2755.0000190734863,
            "volume": 2.0
        }
    })
)]
pub struct AppraisalTotal {
    pub buy:    f64,
    pub sell:   f64,
    pub volume: f32
}
