use sqlx::PgPool;

use crate::{Error, ProjectGroupUuid, ProjectGroup, Result};

pub async fn fetch(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
) -> Result<ProjectGroup> {
    sqlx::query!(
        "
            SELECT
                name,
                description
            FROM project_groups pg
            WHERE pg.id = $1
        ",
            *group_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| ProjectGroup {
            description: x.description,
            name:        x.name,
        })
        .map_err(|e| Error::FetchGroup(e, group_id).into())
}
