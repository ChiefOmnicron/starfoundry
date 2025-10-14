use sqlx::PgPool;

use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::permission::ProjectGroupPermission;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::list_members::ProjectGroupMember;

pub async fn fetch_members_self(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<ProjectGroupMember> {
    let entry = sqlx::query!(
        "
            SELECT
                accepted,
                permission,
                (pg.owner = $2) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.group_id
            WHERE group_id = $1
        ",
            *project_group_uuid,
            *character_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroupMembersSelf(e, project_group_uuid))?;

    let character = eve_gateway_api_client
        .fetch_character(
            character_id,
        )
        .await?;
    Ok(ProjectGroupMember {
        character:      character,
        accepted:       entry.accepted,
        permissions:    ProjectGroupPermission::new(entry.permission),
        is_owner:       entry.is_owner.unwrap_or(false),
    })
}

#[cfg(test)]
mod fetch_members_self_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::test_util::EveGatewayTestApiClient;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("DELETE_AFTER_NEW_MS", "base", "list_default"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();

        let response = super::fetch_members_self(
                &pool,
                &gateway_client,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await;

        assert!(response.is_ok());
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("DELETE_AFTER_NEW_MS", "base", "list_default"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();

        let response = super::fetch_members_self(
                &pool,
                &gateway_client,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await;

        assert!(response.is_err());
    }
}
