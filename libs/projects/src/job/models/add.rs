use serde::Deserialize;
use starfoundry_lib_structures::StructureUuid;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddJobEntry {
    pub type_id:      TypeId,
    pub runs:         i32,
    pub structure_id: StructureUuid,
}
