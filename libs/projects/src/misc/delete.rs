use sqlx::PgPool;

use crate::{Error, ProjectMiscUuid, ProjectUuid, Result};

pub async fn delete(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    market_uuid:  ProjectMiscUuid,
) -> Result<()> {
    sqlx::query!("
            DELETE FROM project_misc
            WHERE project_id = $1
              AND id = $2
        ",
            *project_uuid,
            *market_uuid,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::DeleteMisc(e, project_uuid, market_uuid))
}
