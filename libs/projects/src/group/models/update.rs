use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectGroup {
    pub name:        String,
    pub description: Option<String>,
}
