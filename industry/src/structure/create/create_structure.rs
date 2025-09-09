use serde::Deserialize;
use starfoundry_lib_types::{SystemId, TypeId};
use utoipa::ToSchema;

use crate::structure::models::{Security, StructureType};
use crate::structure::StructureError;
use crate::structure::error::Result;

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
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
pub struct CreateStructure {
    /// Name of the structure
    pub name:              String,
    /// Location of the structure
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

impl CreateStructure {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(StructureError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(StructureError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        if self.structure_id < 1000000000000 {
            return Err(StructureError::ValidationError("Field 'structure_id' must be equal or larger than 1_000_000_000_000".into()));
        };

        Ok(true)
    }
}
