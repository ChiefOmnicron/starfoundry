use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::project_group::{ProjectGroupMember, ProjectGroupPermission};
use starfoundry_lib_industry::ProjectGroupUuid;

use crate::project_group::error::{ProjectGroupError, Result};

pub async fn list_members(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<Vec<ProjectGroupMember>> {
    let entries = sqlx::query!(
        "
            SELECT
                pgm.character_id,
                permission,
                (pg.owner = pgm.character_id) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.project_group_id
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectGroupError::ListMembers(e, project_group_uuid))?;

    let mut members = Vec::new();
    for entry in entries {
        // TODO: add bulk fetch
        let character = match eve_gateway_api_client
            .fetch_character(
                entry.character_id.into(),
            )
            .await {

            Ok(Some(x)) => x,
            Ok(None) => continue,
            Err(_) => continue
        };

        let member = ProjectGroupMember {
            character:      character,
            permissions:    ProjectGroupPermission::new(entry.permission),
            is_owner:       entry.is_owner.unwrap_or(false),
        };
        members.push(member);
    }
    members.sort_by_key(|x| x.character.character_name.clone());

    Ok(members)
}
