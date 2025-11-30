use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{CharacterInfo, EveGatewayApiClient};
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::permission::ProjectGroupPermission;
use crate::project_group::ProjectGroupUuid;

pub async fn list_members(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<Vec<ProjectGroupMember>> {
    let entries = sqlx::query!(
        "
            SELECT
                pgm.character_id,
                accepted,
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
        .map_err(|e| ProjectGroupError::ListGroupMembers(e, project_group_uuid))?;

    let mut members = Vec::new();
    for entry in entries {
        // TODO: add bulk fetch
        let character = eve_gateway_api_client
            .fetch_character(
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "character_name": "SomeCharacterName",
        "character": {
            "character_name": "SomeCharacter",
            "character_id": 1337,
            "corporation_name": "SomeCorporation",
            "corporation_id": 420,
            "alliance_name": "SomeAlliance",
            "alliance_id": 69,
        },
        "accepted": true,
        "permissions": [
            "READ",
            "WRITE_GROUP"
        ],
        "is_owner": false
    })
)]
pub struct ProjectGroupMember {
    pub character:      CharacterInfo,
    pub accepted:       bool,
    pub permissions:    ProjectGroupPermission,
    pub is_owner:       bool,
}
