use serde::Deserialize;

use crate::StructureUuid;

#[derive(Debug, Deserialize)]
pub struct CreateGroup {
    pub name:          String,
    pub structure_ids: Vec<StructureUuid>,
}
