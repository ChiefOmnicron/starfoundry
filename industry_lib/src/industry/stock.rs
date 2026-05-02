use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

#[derive(
    Clone, Debug,
    PartialEq, Eq,
    Deserialize, Serialize,
    ToSchema,
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
