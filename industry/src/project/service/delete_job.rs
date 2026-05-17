use sqlx::PgPool;
use starfoundry_lib_industry::{ProjectJobUuid, ProjectUuid};

use crate::project::error::ProjectError;
use crate::project::error::Result;

pub async fn delete_job(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    job_id:         ProjectJobUuid,
) -> Result<()> {
    sqlx::query!(r#"
            DELETE FROM project_job
            WHERE id = $1
            AND project_id = $2
        "#,
            *job_id,
            *project_id,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| ProjectError::Delete(e, project_id))
}
