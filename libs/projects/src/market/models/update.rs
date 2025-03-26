use serde::Deserialize;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "cost": 1337,
        "quantity": 5,
        "type_id": 73790,
        "source": "1DQ1-A"
    })
)]
pub struct UpdateMarket {
    pub type_id:    TypeId,
    pub quantity:   i32,
    pub cost:       f32,
    pub source:     String,
}
