use serde::Serialize;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;
use crate::ProjectGroupPermission;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectGroupMember {
    // TODO: remove when the fropntend has a component and there is a route
    pub character_name:    String,
    pub character_id:      CharacterId,

    pub accepted:          bool,
    pub permission:        ProjectGroupPermission,
    pub is_owner:          bool,
}
