use sqlx::PgPool;

use crate::{Error, ProjectUuid, Result, UpdateMisc};

pub async fn update(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    update:       UpdateMisc,
) -> Result<()> {
    sqlx::query!("
            UPDATE project_misc
            SET item = $3,
                cost = $4,
                quantity = $5,
                description = $6
            WHERE project_id = $1
              AND id = $2
        ",
            *project_uuid,
            *update.id,
            &update.item,
            &update.cost as _,
            &update.quantity as _,
            &update.description as _,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateMisc(e, project_uuid, update.id))
}
