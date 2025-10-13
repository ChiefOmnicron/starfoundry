use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{fetch_character, ApiClient};

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::list_members::member::ProjectGroupMember;
use crate::project_group::permission::ProjectGroupPermission;
use crate::project_group::ProjectGroupUuid;

pub async fn list_members(
    pool:               &PgPool,
    api_client:         &impl ApiClient,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Vec<ProjectGroupMember>> {
    let entries = sqlx::query!(
        "
            SELECT
                pgm.character_id,
                accepted,
                permission,
                (pg.owner = pgm.character_id) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.group_id
            WHERE group_id = $1
        ",
            *project_group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectGroupError::ListGroupMembers(e, project_group_uuid))?;

    let mut members = Vec::new();
    for entry in entries {
        // TODO: add bulk fetch
        let character = fetch_character(
            api_client,
            entry.character_id.into(),
        )
        .await?;

        let member = ProjectGroupMember {
            character:      character,
            accepted:       entry.accepted,
            permissions:    ProjectGroupPermission::new(entry.permission),
            is_owner:       entry.is_owner.unwrap_or(false),
        };
        members.push(member);
    }
    members.sort_by_key(|x| x.character.character_name.clone());

    Ok(members)
}
