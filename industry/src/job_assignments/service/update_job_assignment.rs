use sqlx::PgPool;
use starfoundry_lib_industry::ProjectJobUuid;

use crate::job_assignments::JobAssignmentUuid;
use crate::job_assignments::error::{JobAssignmentError, Result};

pub async fn update_job_assignment(
    pool:           &PgPool,
    assignment_id:  JobAssignmentUuid,
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
        .map_err(JobAssignmentError::Update)?;

    Ok(())
}
