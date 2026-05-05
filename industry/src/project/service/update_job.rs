use sqlx::PgPool;
use starfoundry_lib_industry::{ProjectJobUuid, ProjectUuid};
use starfoundry_lib_industry::project::UpdateJob;

use crate::project::error::{ProjectError, Result};

pub async fn update_job(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    project_job_id: ProjectJobUuid,
    update:         UpdateJob,
) -> Result<()> {
    // make sure to reset the job if the cost is set reset, otherwise the job
    // detection won't pick it up again
    let job_id = if update.cost.is_none() && update.job_id.is_some() {
        None
    } else {
        update.job_id
    };

    let changes = sqlx::query!("
            UPDATE project_job
            SET
                cost = $3,
                status = $4,
                job_id = $5,
                runs = COALESCE($6, runs)
            WHERE project_id = $1
            AND id = $2
        ",
            *project_id,
            *project_job_id,
            update.cost,
            update.status as _,
            job_id,
            update.runs,
        )
        .execute(pool)
        .await
        .map_err(ProjectError::Update)?;

    if changes.rows_affected() == 0 {
        return Err(ProjectError::JobNotFound(project_id, project_job_id));
    }

    Ok(())
}
