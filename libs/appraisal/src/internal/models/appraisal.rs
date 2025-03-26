use serde::Serialize;
use sqlx::types::Uuid;
use starfoundry_libs_items::Item;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

use super::MarketEntry;

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "code": "rxxBvTowX1",
        "created_at": 1742851510075i64,
        "comment": null,
        "price_modifer": 100,
        "items": [{
            "name": "Tritanium",
            "quantity": 10000,
            "type_id": 34,
            "low_data": false,
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
                "avg": 2.94103448651731,
                "max": 4.03000020980835,
                "min": 0.009999999776482582,
                "total_orders": 11564685569i64
            },
            "sell": {
                "avg": 294.83836382952603,
                "max": 10000.0,
                "min": 4.349999904632568,
                "total_orders": 11300804140i64
            }
        }],
        "invalid": [],
        "market_id": 60003760,
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

    /// comment for the appraisal
    pub comment:        Option<String>,
    /// the price modifier is applied on the return data
    pub price_modifier: i16,
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!(
        {
            "quantity": 1,
            "type_id": 34,
            "low_data": false,
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
                "max": 8.180000305175781,
                "min": 0.019999999552965164,
                "total_orders": 13046927471i64,
                "per_item": {
                    "avg": 2.9603448458144377,
                    "max": 4.090000152587891,
                    "min": 0.009999999776482582
                }
            },
            "sell": {
                "max": 20000.0,
                "min": 8.760000228881836,
                "total_orders": 8652196750i64,
                "per_item": {
                    "avg": 257.6819049365937,
                    "max": 10000.0,
                    "min": 4.380000114440918
                }
            }
        })
)]
pub struct AppraisalItem {
    pub quantity: i64,
    pub type_id:  TypeId,
    /// if there are not enough items on the market to cover the requested amount
    pub low_data: bool,

    /// general item information
    pub meta: Item,

    /// buy information
    pub buy:  MarketEntry,
    /// sell information
    pub sell: MarketEntry,
}
