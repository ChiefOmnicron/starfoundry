use serde::Serialize;
use starfoundry_libs_types::{CategoryId, GroupId, TypeId};
use utoipa::ToSchema;

use crate::ProjectMarketUuid;

#[derive(Debug, Serialize, ToSchema)]
#[serde(transparent)]
pub struct Market(Vec<MarketEntry>);

impl Market {
    pub fn new(entries: Vec<MarketEntry>) -> Self {
        Self(entries)
    }

    pub fn into_group(
        self,
    ) -> Vec<MarketGroup> {
        crate::market::sort_market_by_market_group(self.0)
    }

    pub fn into_inner(
        self
    ) -> Vec<MarketEntry> {
        self.0
    }
}

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "55623aff-c707-4cf3-816e-13110f78f757",
        "quantity": 1,
        "item_name": "Revelation Navy Issue",
        "type_id": 73790,
        "category_id": 6,
        "group_id": 485,
        "cost": 4_200_000,
        "source": "1DQ1-A"
    })
)]
pub struct MarketEntry {
    pub id:          ProjectMarketUuid,
    pub quantity:    i32,
    pub item_name:   String,
    pub type_id:     TypeId,
    pub category_id: CategoryId,
    pub group_id:    GroupId,
    pub cost:        Option<f64>,
    pub source:      Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "header": "UNGROUPED",
        "entries": [{
            "id": "55623aff-c707-4cf3-816e-13110f78f757",
            "quantity": 1,
            "item_name": "Revelation Navy Issue",
            "type_id": 73790,
            "category_id": 6,
            "group_id": 485,
            "cost": 4_200_000,
            "source": "1DQ1-A"
        }]
    })
)]
pub struct MarketGroup {
    pub header:  String,
    pub entries: Vec<MarketEntry>,
}
