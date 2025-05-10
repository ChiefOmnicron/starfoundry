use sqlx::PgPool;

use crate::{Error, ProjectJobUuid, ProjectUuid, Result, UpdateJob};

pub async fn update(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    job_uuid:     ProjectJobUuid,
    update:       UpdateJob,
) -> Result<()> {
    // make sure to reset the job if the cost is set reset, otherwise the job
    // detection won't pick it up again
    let job_id = if update.cost.is_none() && update.job_id.is_some() {
        None
    } else {
        update.job_id
    };

    sqlx::query!("
            UPDATE project_job
            SET cost = $3,
                status = $4,
                job_id = $5
            WHERE project_id = $1
             AND id = $2
        ",
            *project_uuid,
            *job_uuid,
            &update.cost as _,
            &update.status as _,
            job_id as _,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateJobs(e, project_uuid))
}

