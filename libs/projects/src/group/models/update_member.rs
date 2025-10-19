use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectGroupMember {
    pub projects:      String,
    pub project_group: String,
    pub structures:    String,
}
