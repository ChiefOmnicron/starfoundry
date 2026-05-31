use serde::Serialize;
use starfoundry_lib_industry::StructureUuid;

#[derive(Clone, Debug, Serialize)]
pub struct StructureMapping {
    pub structure_uuid: StructureUuid,
    pub category_group: Vec<i32>,
}
