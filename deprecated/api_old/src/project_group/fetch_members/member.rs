use serde::Serialize;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::project_group::permission::ProjectGroupPermission;

#[derive(Debug, Serialize, ToSchema)]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct ProjectGroupMember {
    // TODO: expand like everything else
    pub character_name:    String,
    pub character_id:      CharacterId,

    pub accepted:          bool,
    pub permission:        ProjectGroupPermission,
    pub is_owner:          bool,
}
