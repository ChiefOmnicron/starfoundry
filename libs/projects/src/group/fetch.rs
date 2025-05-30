use sqlx::PgPool;

use crate::{Error, ProjectGroupUuid, ProjectGroup, Result};

pub async fn fetch(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
) -> Result<ProjectGroup> {
    sqlx::query!(
        "
            SELECT
                id,
                name,
                description,
                (
                    SELECT COUNT(*)
                    FROM project_group_member
                    WHERE group_id = $1
                ) AS members,
                (
                    SELECT COUNT(*)
                    FROM project
                    WHERE project_group_id = $1
                ) AS projects
            FROM project_group pg
            WHERE pg.id = $1
        ",
            *group_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| ProjectGroup {
            id:          x.id,
            name:        x.name,
            members:     x.members.unwrap_or_default(),
            projects:    x.projects.unwrap_or_default(),

            description: x.description,
        })
        .map_err(|e| Error::FetchGroup(e, group_id).into())
}
