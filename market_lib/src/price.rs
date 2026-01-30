mod all;

pub use self::all::*;

use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct PriceResponse {
    pub type_id:        TypeId,
    pub adjusted_price: f64,
    pub average_price:  f64,
}
