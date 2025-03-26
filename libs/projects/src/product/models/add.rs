use serde::Deserialize;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddProduct {
    pub quantity:            u32,
    pub material_efficiency: u32,
    pub type_id:             TypeId,
}
