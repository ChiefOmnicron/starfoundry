use serde::Serialize;
use starfoundry_lib_eve_gateway::CharacterInfo;
use utoipa::ToSchema;

use crate::project_group::permission::ProjectGroupPermission;

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
#[schema(
    example = json!({
        "character_name": "SomeCharacterName",
        "character": {
            "character_name": "SomeCharacter",
            "character_id": 1337,
            "corporation_name": "SomeCorporation",
            "corporation_id": 420,
            "alliance_name": "SomeAlliance",
            "alliance_id": 69,
        },
        "accepted": true,
        "permissions": [
            "READ",
            "WRITE_GROUP"
        ],
        "is_owner": false
    })
)]
pub struct ProjectGroupMember {
    pub character:      CharacterInfo,
    pub accepted:       bool,
    pub permissions:    ProjectGroupPermission,
    pub is_owner:       bool,
}
