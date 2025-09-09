use serde::Serialize;
use starfoundry_lib_types::{StructureId, SystemId, TypeId};
use utoipa::ToSchema;

use crate::Security;

#[derive(Clone, Debug, Serialize, ToSchema)]
#[schema(
    example = json!({
        "structure_id": 1003520240,
        "system_id": 30004759,
        "name": "1DQ1-A - 1-st Imperial Palace",
        "type_id": 35834,
        "security": "NULLSEC"
    })
)]
pub struct ResolvedStructure {
    pub structure_id: StructureId,
    pub system_id:    SystemId,
    pub name:         String,
    pub type_id:      TypeId,
    pub security:     Security,
}
