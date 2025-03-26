use sqlx::PgPool;

use crate::{Error, ProjectGroupUuid, UpdateProjectGroup, Result};

pub async fn update(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
    info:     UpdateProjectGroup,
) -> Result<()> {
    sqlx::query!("
        UPDATE project_groups
        SET
            name = $2,
            description = $3
        WHERE id = $1
    ",
        *group_id,
        info.name,
        info.description,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| Error::UpdateGroup(e, group_id).into())
}
