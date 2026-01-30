use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::Structure;
use starfoundry_lib_types::CharacterId;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::structure::service::FetchStructureQuery;

pub async fn list_default_market(
    pool:                   &PgPool,
    character_id:           CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<Vec<Structure>> {
    let markets = sqlx::query!("
            SELECT structure_id
            FROM project_group_default_market
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroupDefaults(e, project_group_uuid))?
        .into_iter()
        .map(|x| x.structure_id.into())
        .collect::<Vec<_>>();

    let mut structures = Vec::new();
    for market in markets {
        if let Some(x) = crate::structure::service::fetch(
            pool,
            eve_gateway_api_client,
            character_id,
            market,
            FetchStructureQuery::default(),
        ).await? {
            structures.push(x);
        } else {
            // silently ignore errors
            tracing::debug!("Couldn't find structure {}", market);
            continue;
        }
    }

    Ok(structures)
}

#[cfg(test)]
mod list_default_market_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::eve_gateway_api_client;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::list_default_market(
                &pool,
                CharacterId(1),
                &eve_gateway_api_client().unwrap(),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 1);
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
        let response = super::list_default_market(
                &pool,
                CharacterId(1),
                &eve_gateway_api_client().unwrap(),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 0);
    }
}
