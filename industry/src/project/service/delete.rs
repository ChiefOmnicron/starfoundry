use sqlx::PgPool;
use starfoundry_lib_industry::ProjectUuid;

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
