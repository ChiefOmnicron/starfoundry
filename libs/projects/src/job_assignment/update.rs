use sqlx::PgPool;

use crate::{Error, ProjectJobAssignmentUuid, ProjectJobUuid, Result};

pub async fn update_job_state(
    pool:          &PgPool,
    assignment_id: ProjectJobAssignmentUuid,
    job_id:        ProjectJobUuid,
) -> Result<()> {
    sqlx::query!("
            UPDATE project_job_assignments
            SET started = true
            WHERE id = $1
              AND job_id = $2
        ",
            *assignment_id,
            *job_id,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateProjectJobAssignmentJob(e, assignment_id, job_id))
}
