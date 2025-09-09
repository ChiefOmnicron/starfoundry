use serde::Deserialize;
use starfoundry_lib_structures::{StructureGroupUuid, StructureUuid};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::{AddProduct, ProjectGroupUuid, StockMinimal};

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My Project Name",
        "orderer": "its for somebody I used to know",
        "project_group_id": "60f5931b-bea5-45a1-a2ea-520d2535b138",
        "structure_group_id": "caf60b7a-5abd-41b7-bc01-721f049def56",
        "note": null,
        "sell_price": 4_200_000_000f64,

        "blacklist": [
            4312, 4247, 4051, 4246
        ],

        "markets": [
            "00000000-0000-0000-0000-000000000001"
        ],

        "products": [
            {
                "quantity": 1,
                "material_efficiency": 0,
                "type_id": 73790,
                "item_name": "Revelation Navy Issue"
            }
        ],
        "additional_products": [
            {
                "quantity": 50_000,
                "type_id": 16274,
            }
        ],

        "stocks": [{
            "quantity": 3315,
            "type_id": 11399,
        }]
    })
)]
pub struct CreateProject {
    pub name:                String,
    /// products that should be build in the project
    pub products:            Vec<AddProduct>,
    pub structure_group_id:  StructureGroupUuid,
    pub project_group_id:    ProjectGroupUuid,

    pub orderer:             Option<String>,
    pub note:                Option<String>,
    pub sell_price:          Option<f64>,

    pub markets:             Vec<StructureUuid>,

    /// products that either shouldn't be build or cannot be build, but still
    /// belong to the project
    #[serde(default)]
    pub additional_products: Vec<AdditionalProduct>,
    #[serde(default)]
    pub stocks:              Vec<StockMinimal>,
    #[serde(default)]
    pub blacklist:           Vec<TypeId>,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct AdditionalProduct {
    pub quantity: u32,
    pub type_id:  TypeId,
}
