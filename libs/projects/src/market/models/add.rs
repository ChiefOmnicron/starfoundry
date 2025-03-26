use serde::Deserialize;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "type_id": 73790,
        "quantity": 1,
        "source": "1DQ1-A",
        "cost": 1337
    })
)]
pub struct AddMarket {
    pub type_id:  TypeId,
    pub quantity: i32,
    pub source:   Option<String>,
    pub cost:     Option<f32>,
}
