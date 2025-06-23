use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::{Error, ProjectGroupUuid, ProjectGroup, Result};

pub async fn fetch(
    pool:         &PgPool,
    character_id: CharacterId,
    group_id:     ProjectGroupUuid,
) -> Result<ProjectGroup> {
    sqlx::query!(
        "
            SELECT
                id,
                name,
                description,
                owner = $1 AS is_owner,
                (
                    SELECT COUNT(*)
                    FROM project_group_member
                    WHERE group_id = $2
                ) AS members,
                (
                    SELECT COUNT(*)
                    FROM project
                    WHERE project_group_id = $2
                ) AS projects
            FROM project_group pg
            WHERE pg.id = $2
        ",
            *character_id,
            *group_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| ProjectGroup {
            id:          x.id,
            name:        x.name,
            members:     x.members.unwrap_or(1),
            projects:    x.projects.unwrap_or(0),
            is_owner:    x.is_owner.unwrap_or_default(),

            description: x.description,
        })
        .map_err(|e| Error::FetchGroup(e, group_id).into())
}
