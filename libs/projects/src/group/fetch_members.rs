use sqlx::PgPool;

use crate::{Error, ProjectGroupUuid, ProjectGroupMember, Result};

pub async fn fetch_members(
    pool:       &PgPool,
    group_uuid: ProjectGroupUuid,
) -> Result<Vec<ProjectGroupMember>> {
    sqlx::query!(
        "
            SELECT
                character_name,
                c.character_id,
                accepted,
                projects,
                project_group,
                structures,
                (pg.owner = c.character_id) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.group_id
            JOIN character c ON c.character_id = pgm.character_id
            WHERE group_id = $1
            ORDER BY character_name ASC
        ",
            *group_uuid,
        )
        .fetch_all(pool)
        .await
        .map(|entries| {
            entries
                .into_iter()
                .map(|x| ProjectGroupMember {
                    character_name: x.character_name,
                    character_id:   x.character_id.into(),

                    accepted:       x.accepted,
                    projects:       x.projects,
                    project_group:  x.project_group,
                    structures:     x.structures,
                    is_owner:       x.is_owner.unwrap_or(false),
                })
                .collect::<Vec<_>>()
        })
        .map_err(|e| Error::ListGroupMembers(e, group_uuid).into())
}
