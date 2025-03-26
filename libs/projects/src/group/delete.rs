use sqlx::PgPool;

use crate::{Error, ProjectGroupUuid, Result};

pub async fn delete(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
) -> Result<ProjectGroupUuid> {
    sqlx::query!("
        DELETE FROM project_groups
        WHERE id = $1
        RETURNING id
    ",
        *group_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| ProjectGroupUuid::new(x.id))
    .map_err(|e| Error::DeleteGroup(e, group_id).into())
}
