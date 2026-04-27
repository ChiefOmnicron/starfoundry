use sqlx::PgPool;

use crate::project::error::{ProjectError, Result};
use crate::project::service::ProjectJobUuid;
use crate::project::ProjectAssignmentUuid;

pub async fn update_job_assignment(
    pool:           &PgPool,
    assignment_id:  ProjectAssignmentUuid,
    job_id:         ProjectJobUuid,
) -> Result<()> {
    sqlx::query!("
            UPDATE project_job_assignment
            SET started = TRUE
            WHERE id = $1
            AND job_id = $2
        ",
            *assignment_id,
            *job_id,
        )
        .execute(pool)
        .await
        .map_err(ProjectError::Update)?;

    Ok(())
}
