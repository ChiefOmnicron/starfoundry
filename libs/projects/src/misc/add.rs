use sqlx::PgPool;

use crate::{AddMisc, Error, ProjectUuid, Result};

pub async fn add(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    entry:        AddMisc,
) -> Result<()> {
    sqlx::query!("
            INSERT INTO project_misc
            (
                project_id,
                item,
                cost,
                quantity,
                description
            )
            VALUES ($1, $2, $3, $4, $5)
        ",
            *project_uuid,
            entry.item,
            entry.cost,
            entry.quantity,
            entry.description,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::AddMisc(e, project_uuid))
}
