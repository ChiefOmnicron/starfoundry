use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{Error, Result};

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
#[deprecated]
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
#[deprecated]
pub struct CreateProjectGroup {
    /// Maximum length 100
    pub name:        String,
    /// Maximum length 10_000
    pub description: Option<String>,
}

impl CreateProjectGroup {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(Error::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(Error::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        match &self.description {
            Some(x) => {
                if x.len() >= 10_000 {
                    return Err(Error::ValidationError("Field 'description' is too long, max length: 10_000".into()));
                }
                Some(x)
            },
            None => None,
        };

        Ok(true)
    }
}
