use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "b034c3a9-2f4d-487d-95bb-c66fc20148b3",
        "name": "My cool group",
        "members": 5,
        "projects": 100,
        "is_owner": true,
        "description": "Bunch of cool projects"
    })
)]
pub struct ProjectGroup {
    pub id:          Uuid,
    pub name:        String,
    pub members:     i64,
    pub projects:    i64,
    pub is_owner:    bool,

    pub description: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool group",
        "description": "Bunch of cool projects"
    })
)]
pub struct CreateProjectGroup {
    /// Maximum length 100
    pub name:        String,
    /// Maximum length 10_000
    pub description: Option<String>,
}
