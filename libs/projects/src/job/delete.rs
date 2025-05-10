use sqlx::PgPool;

use crate::{Error, ProjectJobUuid, ProjectUuid, Result};

pub async fn delete(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    job_uuid:     ProjectJobUuid,
) -> Result<()> {
    sqlx::query!("
            DELETE FROM project_job
            WHERE project_id = $1
              AND id = $2
        ",
            *project_uuid,
            *job_uuid,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::DeleteJob(e, project_uuid, job_uuid))
}
