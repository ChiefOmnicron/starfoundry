use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

use crate::StructureDynamicGroupUuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct StructureDynamicGroup {
    pub id:        Option<StructureDynamicGroupUuid>,
    pub name:      String,

    #[serde(default)]
    pub group_ids: Vec<Uuid>,
}

impl fmt::Display for StructureDynamicGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
