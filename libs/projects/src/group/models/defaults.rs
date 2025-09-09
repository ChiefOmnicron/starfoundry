use serde::{Deserialize, Serialize};
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;
use starfoundry_lib_structures::StructureUuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "markets": [
            "b54c6098-532b-4a28-bd1b-83b4dbe55d12"
        ],
        "blacklist": [
            4051,
            4246,
            4247,
            4312
        ],
    })
)]
#[deprecated]
pub struct ProjectGroupDefault {
    /// Reference to a structure
    /// Does NOT use the game internal id
    #[serde(default)]
    pub markets:   Vec<StructureUuid>,

    /// TypeId of items that should not be build
    #[serde(default)]
    pub blacklist: Vec<TypeId>,
}
