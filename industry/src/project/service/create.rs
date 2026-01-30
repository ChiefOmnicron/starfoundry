use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::project_group::ProjectGroupUuid;
use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    project_info: CreateProject,
) -> Result<ProjectUuid> {
    project_info.validate()?;

    let project_id = sqlx::query!(r#"
            INSERT INTO project
            (
                owner,
                sell_price,
                project_group_id,
                orderer,
                name
            )
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id
        "#,
            *character_id,
            project_info.sell_price,
            *project_info.project_group_id,
            project_info.orderer,
            project_info.name,
        )
        .fetch_one(pool)
        .await
        .map_err(ProjectError::CreateProject)?;

    Ok(project_id.id.into())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateProject {
    pub sell_price:       Option<f64>,
    pub project_group_id: ProjectGroupUuid,
    pub orderer:          String,
    pub name:             String,
}

impl CreateProject {
    pub fn validate(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(ProjectError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(ProjectError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        Ok(true)
    }
}
