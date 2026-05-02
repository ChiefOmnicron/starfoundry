use serde::Deserialize;
use starfoundry_lib_industry::{ProjectGroupUuid, ProjectUuid};
use utoipa::ToSchema;
use sqlx::PgPool;

use crate::project::error::{ProjectError, Result};
use crate::project::service::ProjectStatus;

pub async fn update(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    update:         UpdateProjectRequest,
) -> Result<()> {
    let result = sqlx::query!("
            UPDATE project
            SET
                sell_price = $2,
                project_group_id = $3,
                orderer = $4,
                name = $5,
                note = $6,
                status = $7
            WHERE id = $1
        ",
            *project_id,
            update.sell_price,
            *update.project_group_id,
            update.orderer,
            update.name,
            update.note,
            update.status as _,
        )
        .execute(pool)
        .await
        .map_err(ProjectError::Update)?;

    if result.rows_affected() == 0 {
        return Err(ProjectError::NotFound(project_id));
    }

    Ok(())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectRequest {
    pub project_group_id:   ProjectGroupUuid,
    pub orderer:            String,
    pub name:               String,
    pub status:             ProjectStatus,

    pub sell_price:         Option<f64>,
    pub note:               Option<String>,
}
