use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct ProjectGroup {
    pub id:          Uuid,
    pub name:        String,
    pub members:     i64,
    pub projects:    i64,

    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProjectGroup {
    pub name:        String,
    pub description: Option<String>,
}
