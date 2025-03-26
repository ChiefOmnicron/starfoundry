use serde::Serialize;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, ToSchema)]
pub struct MarketPrice {
    pub source:    String,
    pub type_id:   i32,
    pub quantity:  u64,
    pub remaining: u64,
    pub price:     f64,
}
