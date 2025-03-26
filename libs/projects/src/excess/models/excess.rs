use serde::Serialize;
use starfoundry_libs_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(transparent)]
pub struct Excess(Vec<ExcessEntry>);

impl Excess {
    pub fn new(entries: Vec<ExcessEntry>) -> Self {
        Self(entries)
    }

    pub fn into_group(
        self,
    ) -> Vec<ExcessGroup> {
        crate::excess::sort_excess_by_market_group(self.0)
    }

    pub fn into_inner(
        self
    ) -> Vec<ExcessEntry> {
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
pub struct ExcessEntry {
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
pub struct ExcessGroup {
    pub header:  String,
    pub entries: Vec<ExcessEntry>,
}
