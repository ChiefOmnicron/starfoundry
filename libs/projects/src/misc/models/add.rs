use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "item": "Hauling",
        "cost": 1337,
        "quantity": null,
        "description": "JF Fuel cost Jita > 1DQ"
    })
)]
pub struct AddMisc {
    pub item:        String,
    pub cost:        f64,
    pub quantity:    Option<i32>,
    pub description: Option<String>,
}
