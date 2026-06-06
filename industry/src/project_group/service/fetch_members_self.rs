use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::project_group::{ProjectGroupMember, ProjectGroupPermission};
use starfoundry_lib_industry::ProjectGroupUuid;
use starfoundry_lib_types::CharacterId;

use crate::project_group::error::{ProjectGroupError, Result};

pub async fn fetch_members_self(
    pool:                   &PgPool,
    character_id:           CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<Option<ProjectGroupMember>> {
    let entry = sqlx::query!(
        "
            SELECT
                permission,
                (pg.owner = $2) AS is_owner
            FROM project_group_member pgm
            JOIN project_group pg ON pg.id = pgm.project_group_id
            WHERE project_group_id = $1
            AND character_id = $2
        ",
            *project_group_uuid,
            *character_id,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchMembersSelf(e, project_group_uuid))?;

    match eve_gateway_api_client
        .fetch_character(
            character_id,
        )
        .await {

        Ok(Some(x)) => Ok(Some(ProjectGroupMember {
            character:      x,
            permissions:    ProjectGroupPermission::new(entry.permission),
            is_owner:       entry.is_owner.unwrap_or(false),
        })),
        Ok(None) => Ok(None),
        Err(_) => Ok(None),
    }
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
            scripts("base"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();

        let response = super::fetch_members_self(
                &pool,
                CharacterId(1),
                &gateway_client,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await;

        assert!(response.is_ok());
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();

        let response = super::fetch_members_self(
                &pool,
                CharacterId(1),
                &gateway_client,
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await;

        assert!(response.is_err());
    }
}
