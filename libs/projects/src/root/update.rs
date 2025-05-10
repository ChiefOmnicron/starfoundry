use sqlx::PgPool;
use uuid::Uuid;

use crate::{Error, ProjectUuid, Result, UpdateProject};

pub async fn update(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    info:         UpdateProject,
) -> Result<()> {
    sqlx::query!("
            UPDATE project
            SET
                name = $2,
                status = $3,
                sell_price = $4,
                orderer = $5,
                note = $6,
                project_group_id = $7
            WHERE id = $1
        ",
            *project_uuid,
            info.name,
            &info.status as _,
            &info.sell_price as _,
            &info.orderer as _,
            &info.note as _,
            *info.project_group_id.unwrap_or(Uuid::default().into()),
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateProject(e, project_uuid))
}
