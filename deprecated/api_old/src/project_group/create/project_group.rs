use crate::project_group::error::{Error, Result};
use serde::Deserialize;
use utoipa::ToSchema;

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
