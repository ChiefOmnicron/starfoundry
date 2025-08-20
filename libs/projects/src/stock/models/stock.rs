use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(transparent)]
pub struct Stock(Vec<StockEntry>);

impl Stock {
    pub fn new(entries: Vec<StockEntry>) -> Self {
        Self(entries)
    }

    pub fn into_group(
        self,
    ) -> Vec<StockGroup> {
        crate::stock::sort_stock_by_market_group(self.0)
    }

    pub fn into_inner(
        self
    ) -> Vec<StockEntry> {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!([{
        "item_name": "Morphite",
        "quantity": 3315,
        "type_id": 11399,
        "category_id": 4,
        "group_id": 18
    }, {
        "item_name": "Pyerite",
        "quantity": 637895,
        "type_id": 35,
        "category_id": 4,
    }])
)]
pub struct StockEntry {
    pub item_name:   String,
    pub quantity:    i32,
    pub cost:        Option<f64>,
    pub type_id:     TypeId,
    pub category_id: CategoryId,
    pub group_id:    GroupId,
}

/// Stocks grouped by their category_id and group_id
#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!([{
        "header": "MINERALS",
        "entries": [{
            "item_name": "Morphite",
            "quantity": 3315,
            "type_id": 11399,
            "category_id": 4,
            "group_id": 18
        }],
    }, {
        "header": "MOON_MATERIALS",
        "entries": [{
            "item_name": "Cadmium",
            "quantity": 196,
            "type_id": 16643,
            "category_id": 4,
            "group_id": 427
        }]
    }])
)]
pub struct StockGroup {
    pub header:  String,
    pub entries: Vec<StockEntry>,
}

#[derive(
    Clone, Debug,
    PartialEq, Eq,
    Deserialize, Serialize,
    ToSchema
)]
#[schema(
    example = json!({
        "quantity": 3315,
        "type_id": 11399,
    })
)]
pub struct StockMinimal {
    pub quantity:    i32,
    pub type_id:     TypeId,
}
