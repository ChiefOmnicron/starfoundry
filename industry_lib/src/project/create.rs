use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{Error, ProjectGroupUuid, ProjectUuid, Result};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CreateProject {
    pub project_group_id:   ProjectGroupUuid,
    pub orderer:            String,
    pub name:               String,

    pub sell_price:         Option<f64>,
    pub notes:              Option<String>,

    pub pre_products:       Option<String>,
    pub pre_additional:     Option<String>,
}

impl CreateProject {
    pub fn validate(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(Error::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(Error::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        Ok(true)
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "fd324c9f-ecda-49c8-948e-18f4b4b23bff"
    })
)]
pub struct CreateProjectResponse {
    pub id: ProjectUuid,
}

impl Default for CreateProjectResponse {
    fn default() -> Self {
        Self {
            id: Uuid::default().into(),
        }
    }
}
