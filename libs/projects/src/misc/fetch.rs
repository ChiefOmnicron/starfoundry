use sqlx::PgPool;

use crate::{Error, Misc, ProjectUuid, Result};

pub async fn fetch(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Vec<Misc>> {
    sqlx::query_as!(
        Misc,
        r#"
            SELECT
                id,
                item,
                quantity,
                cost,
                description
            FROM project_misc
            WHERE project_id = $1
            ORDER BY item, cost, description
        "#,
            *project_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchMisc(e, project_uuid))
}
