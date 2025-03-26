use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, ToSchema)]
pub struct MarketEntry {
    pub max:          f64,
    pub min:          f64,

    pub total_orders: i64,

    pub per_item:     MarketEntyPerItem,
}

#[derive(Clone, Debug, Default, Serialize, ToSchema)]
pub struct MarketEntyPerItem {
    pub avg:    f64,
    pub max:    f64,
    pub min:    f64,
}
