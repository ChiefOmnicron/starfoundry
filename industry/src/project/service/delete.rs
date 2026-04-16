use sqlx::PgPool;

use crate::project::ProjectUuid;
use crate::project::error::ProjectError;
use crate::project::error::Result;

pub async fn delete(
    pool:           &PgPool,
    project_id:     ProjectUuid,
) -> Result<()> {
    sqlx::query!(r#"
            DELETE FROM project
            WHERE id = $1
        "#,
            *project_id,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| ProjectError::Delete(e, project_id))
}
