use serde::Deserialize;
use utoipa::ToSchema;

use crate::{Error, Result};

#[derive(Debug, Deserialize, ToSchema)]
#[deprecated]
pub struct UpdateProjectGroup {
    pub name:        String,
    pub description: Option<String>,
}

impl UpdateProjectGroup {
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
