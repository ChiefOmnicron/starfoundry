use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ProjectGroup {
    pub name:        String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectGroup {
    pub name:        String,
    pub description: Option<String>,
}
