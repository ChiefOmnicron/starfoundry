use serde::Serialize;
use starfoundry_libs_types::CharacterId;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectGroupMember {
    // TODO: remove when the fropntend has a component and there is a route
    pub character_name:    String,
    pub character_id:      CharacterId,

    pub accepted:          bool,
    pub projects:          String,
    pub project_group:     String,
    pub structures:        String,
    pub is_owner:          bool,
}
