use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct MarketPrice {
    /// Type ID of the item
    pub type_id:       TypeId,
    /// Adjusted price of the item
    pub adjusted_price: f64,
    /// Average price of the item
    #[serde(default)]
    pub average_price:  f64,
}
