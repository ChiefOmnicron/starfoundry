use serde::Serialize;
use starfoundry_libs_types::CharacterId;
use utoipa::ToSchema;

use crate::project_group::permission::ProjectGroupPermission;

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[schema(
    example = json!({
        "character_name": "SomeCharacterName",
        "character_id": 1337,

        "accepted": true,
        "permissions": [
            "READ",
            "WRITE_GROUP"
        ],
        "is_owner": false
    })
)]
pub struct ProjectGroupMember {
    // TODO: expand like everything else
    pub character_name:    String,
    pub character_id:      CharacterId,

    pub accepted:          bool,
    pub permissions:       ProjectGroupPermission,
    pub is_owner:          bool,
}
