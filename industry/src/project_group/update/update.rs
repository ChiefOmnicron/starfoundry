use serde::Deserialize;
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};

#[derive(Debug, Deserialize, ToSchema)]
#[cfg_attr(test, derive(serde::Serialize))]
pub struct UpdateProjectGroup {
    pub name:        String,
    pub description: Option<String>,
}

impl UpdateProjectGroup {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(ProjectGroupError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(ProjectGroupError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        match &self.description {
            Some(x) => {
                if x.len() >= 10_000 {
                    return Err(ProjectGroupError::ValidationError("Field 'description' is too long, max length: 10_000".into()));
                }
                Some(x)
            },
            None => None,
        };

        Ok(true)
    }
}
