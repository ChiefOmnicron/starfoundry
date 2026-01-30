mod internal;

pub use self::internal::*;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use starfoundry_lib_eve_gateway::{Item, StructurePosition, StructureRigResponse, StructureServiceResponse, StructureType, System};
use starfoundry_lib_types::starfoundry_uuid;

starfoundry_uuid!(StructureUuid, "StructureUuid");

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "15bd47e3-6b38-4cc1-887b-94924fff30a1",
        "name": "1DQ1-A - RIP",
        "structure_id": 1337,
        "structure_type": "AZBEL",
        "system": {
            "constellation_id": 20000696,
            "constellation_name": "O-EIMK",
            "region_id": 10000060,
            "region_name": "Delve",
            "system_id": 30004759,
            "system_name": "1DQ1-A",
            "security": -0.38578233,
            "security_group": "NULLSEC",
        },
        "item": {
            "base_price": null,
            "category_id": 65,
            "group_id": 1657,
            "meta_group_id": 1,
            "name": "Keepstar",
            "repackaged": null,
            "type_id": 35834,
            "volume": 800000
        },
        "rigs": [],
        "service": [{
            "base_price": null,
            "category_id": 66,
            "group_id": 1321,
            "meta_group_id": 54,
            "name": "Standup Market Hub I",
            "repackaged": null,
            "type_id": 35892,
            "volume": 32000
        }]
    })
)]
pub struct Structure {
    /// Internal id of the structure
    pub id:                   StructureUuid,
    /// EVE Id of the structure
    pub structure_id:         i64,
    /// Name of the structure
    pub name:                 String,
    /// Location of the structure
    pub system:               System,
    /// Type information
    pub item:                 Item,
    /// List of all rigs that are in the structure
    pub rigs:                 Vec<StructureRigResponse>,
    /// Id of the structure in-game
    pub services:             Vec<Item>,
    /// Position of the structure in the system
    pub position:             StructurePosition,
    /// Type of the structure
    pub structure_type:       StructureType,

    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installable_rigs:     Option<Vec<StructureRigResponse>>,
    #[serde(skip_deserializing)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installable_services: Option<StructureServiceResponse>,
}
