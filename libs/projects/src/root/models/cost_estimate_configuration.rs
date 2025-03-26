use serde::Deserialize;
use starfoundry_libs_structures::StructureGroupUuid;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

use crate::StockMinimal;

#[derive(Debug, Deserialize, ToSchema)]
pub struct CostEstimateConfiguration {
    pub products:            Vec<CostEstimateProduct>,
    #[serde(default)]
    pub stocks:              Vec<StockMinimal>,

    pub structure_group:     StructureGroupUuid,
}

/// information about a product being built
/// 
#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "quantity": 1,
        "material_efficiency": 0,
        "type_id": 73790
    })
)]
pub struct CostEstimateProduct {
    pub quantity:            u32,
    pub type_id:             TypeId,
    #[serde(default)]
    pub material_efficiency: Option<u32>,
}
