use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::{ProjectGroup, list_default_blacklist, list_default_market, list_members};

pub async fn fetch(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<Option<ProjectGroup>> {
    let entry = sqlx::query!("
            SELECT
                id,
                name,
                description,
                owner = $1 AS is_owner
                --(
                --    SELECT COUNT(*)
                --    FROM project
                --    WHERE project_group_id = $2
                --) AS projects
            FROM project_group pg
            WHERE pg.id = $2
        ",
            *character_id,
            *project_group_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroup(e, project_group_uuid))?;

    if let Some(x) = entry {
        let project_group = ProjectGroup {
            id:                x.id.into(),
            name:              x.name,
            //project_count:     x.projects.unwrap_or(0),
            project_count:     0,
            is_owner:          x.is_owner.unwrap_or_default(),
            description:       x.description,

            default_blacklist: list_default_blacklist(
                pool,
                eve_gateway_api_client,
                project_group_uuid
            ).await?,
            default_market:    list_default_market(
                pool,
                character_id,
                eve_gateway_api_client,
                project_group_uuid
            ).await?,
            members:           list_members(
                pool,
                eve_gateway_api_client,
                project_group_uuid
            ).await?,
        };
        Ok(Some(project_group))
    } else {
        Ok(None)
    }
}

#[cfg(test)]
mod fetch_project_group_test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
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

        let response = super::fetch(
                &pool,
                &gateway_client,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        let response = response.unwrap();
        assert_eq!(response.is_owner, true);
        assert_eq!(response.name, "First".to_string());
        assert_eq!(response.description, Some("Description".into()));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn no_entry_with_default_uuid(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();

        let response = super::fetch(
                &pool,
                &gateway_client,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await;

        assert!(response.unwrap().is_none());
    }
}
