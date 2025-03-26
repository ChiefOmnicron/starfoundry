use serde::{Deserialize, Serialize};
use starfoundry_libs_types::{SystemId, TypeId};
use utoipa::ToSchema;

use crate::{Security, StructureType};

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "d187ee9c-3afe-4e72-a5b4-646020b0d646",
        "name": "1DQ1-A - 1-st Imperial Palace",
        "rigs": [
            37275
        ],
        "security": "NULLSEC",
        "services": [
            35894
        ],
        "structure_id": 1003520240,
        "structure_type_id": 35834,
        "system_id": 30004759
    })
)]
pub struct UpdateStructure {
    /// Name of the structure
    pub name:              String,
    /// Location of the strucutre
    pub system_id:         SystemId,
    /// Security of the location the structure is in
    pub security:          Security,

    /// Type of structure
    #[serde(deserialize_with = "StructureType::deserialize")]
    #[serde(serialize_with = "StructureType::serialize")]
    #[serde(rename = "structure_type_id")]
    #[schema(
        value_type = TypeId,
        example = "565692"
    )]
    pub structure_type:    StructureType,
    /// List of all rigs that are in the structure
    pub rigs:              Vec<TypeId>,
    /// Id of the structure in-game
    pub services:          Vec<TypeId>,

    /// EVE Id of the structure
    pub structure_id:      i64,
}
