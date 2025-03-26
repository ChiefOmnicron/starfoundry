use serde::Deserialize;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "quantity": 5,
        "type_id": 73790
    })
)]
pub struct UpdateMineral {
    pub type_id:    TypeId,
    pub quantity:   i32,
}
