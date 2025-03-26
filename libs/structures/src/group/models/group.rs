use serde::Serialize;

use crate::{StructureGroupUuid, StructureUuid};

#[derive(Debug, Serialize)]
pub struct StructureGroup {
    pub id:            StructureGroupUuid,
    pub name:          String,
    pub structure_ids: Vec<StructureUuid>,
}
