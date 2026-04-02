use serde::Deserialize;
use utoipa::ToSchema;
use sqlx::PgPool;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use crate::project::service::{ProjectJobStatusDatabase, ProjectJobUuid};

pub async fn update_job(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    project_job_id: ProjectJobUuid,
    update:         UpdateProjectJob,
) -> Result<()> {
    let changes = sqlx::query!("
            UPDATE project_job
            SET
                cost = $3,
                status = $4
            WHERE project_id = $1
            AND id = $2
        ",
            *project_id,
            *project_job_id,
            update.cost,
            update.status as _,
        )
        .execute(pool)
        .await
        .map_err(ProjectError::Update)?;

    if changes.rows_affected() == 0 {
        return Err(ProjectError::JobNotFound(project_id, project_job_id));
    }

    Ok(())
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateProjectJob {
    pub cost:   Option<f64>,
    pub status: ProjectJobStatusDatabase,
}
