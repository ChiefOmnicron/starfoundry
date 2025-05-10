use serde::Serialize;
use starfoundry_libs_structures::StructureGroupUuid;
use utoipa::ToSchema;

use crate::{Product, Finance, ProjectGroupUuid, ProjectStatus};

#[derive(Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My Project Name",
        "status": "DONE",
        "orderer": "its for somebody I used to know",
        "project_group_id": "60f5931b-bea5-45a1-a2ea-520d2535b138",
        "structure_group_id": "caf60b7a-5abd-41b7-bc01-721f049def56",
        "note": null,

        "products": [
            {
                "quantity": 1,
                "material_efficiency": 0,
                "type_id": 73790,
                "item_name": "Revelation Navy Issue"
            }
        ],

        "finance": {
            "excess": 1276522,
            "jobs": 182774340,
            "market": 1988189400,
            "misc": 100000,
            "stock": 0,
            "sell_price": 4_200_000_000f64,
        }
    })
)]
pub struct Project {
    pub name:               String,
    pub status:             ProjectStatus,
    pub orderer:            String,
    pub project_group_id:   ProjectGroupUuid,
    pub structure_group_id: StructureGroupUuid,

    pub note:               Option<String>,

    pub products:           Vec<Product>,
    pub finance:            Finance
}
