use sqlx::PgPool;

use crate::{Error, ProjectUuid, Result};

pub async fn delete(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<()> {
    sqlx::query!("
            DELETE FROM project
            WHERE id = $1
        ",
            *project_uuid,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::DeleteProject(e, project_uuid))
}
