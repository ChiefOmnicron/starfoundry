use serde::{Deserialize, Serialize};
use starfoundry_libs_structures::StructureUuid;
use starfoundry_libs_types::TypeId;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectGroupDefault {
    #[serde(default)]
    pub markets:   Vec<StructureUuid>,

    #[serde(default)]
    pub blacklist: Vec<TypeId>,
}
