use serde::Serialize;

use crate::StructureUuid;

#[derive(Clone, Debug, Serialize)]
pub struct StructureMapping {
    pub structure_uuid: StructureUuid,
    pub category_group: Vec<i32>,
}
