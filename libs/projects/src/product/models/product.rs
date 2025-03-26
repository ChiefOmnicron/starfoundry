use serde::{Deserialize, Serialize};
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

/// information about a product being built
/// 
#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "quantity": 1,
        "material_efficiency": 0,
        "type_id": 73790
    })
)]
pub struct Product {
    pub quantity:            i32,
    #[serde(default)]
    pub material_efficiency: i32,
    pub type_id:             TypeId,
}
