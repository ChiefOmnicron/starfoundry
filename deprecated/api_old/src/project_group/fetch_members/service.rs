use sqlx::PgPool;

use crate::project_group::error::{Error, Result};
use crate::project_group::fetch_members::member::ProjectGroupMember;
use crate::project_group::permission::ProjectGroupPermission;
use crate::project_group::ProjectGroupUuid;

pub async fn fetch_members(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Vec<ProjectGroupMember>> {
    sqlx::query!(
        "
            SELECT
                character_name,
                c.character_id,
                accepted,
                permission,
                (pg.owner = c.character_id) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.group_id
            JOIN character c ON c.character_id = pgm.character_id
            WHERE group_id = $1
            ORDER BY character_name ASC
        ",
            *project_group_uuid,
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
                    permission:     ProjectGroupPermission::new(x.permission),
                    is_owner:       x.is_owner.unwrap_or(false),
                })
                .collect::<Vec<_>>()
        })
        .map_err(|e| Error::ListGroupMembers(e, project_group_uuid).into())
}
