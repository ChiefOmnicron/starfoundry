use serde::Deserialize;
use utoipa::ToSchema;

use crate::ProjectMiscUuid;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "id": "25507fee-0531-4dd1-8e3c-8b83b24fefba",
        "item": "Hauling",
        "cost": 1337,
        "quantity": null,
        "description": "JF Fuel cost Jita > 1DQ"
    })
)]
pub struct UpdateMisc {
    pub id:          ProjectMiscUuid,
    pub item:        String,
    pub cost:        f32,
    pub quantity:    Option<i32>,
    pub description: Option<String>,
}
