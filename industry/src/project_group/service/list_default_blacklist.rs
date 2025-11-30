use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn list_default_blacklist(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_group_uuid:     ProjectGroupUuid,
) -> Result<Vec<Item>> {
    let type_ids = sqlx::query!("
            SELECT type_id
            FROM project_group_default_blacklist
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroupDefaults(e, project_group_uuid))?
        .into_iter()
        .map(|x| x.type_id.into())
        .collect::<Vec<_>>();

    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?;

    Ok(items)
}

#[cfg(test)]
mod list_default_blacklist_project_group_test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::eve_gateway_api_client;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base", "list_default"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::list_default_blacklist(
                &pool,
                &eve_gateway_api_client().unwrap(),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 4);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base", "list_default"),
        ),
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let response = super::list_default_blacklist(
                &pool,
                &eve_gateway_api_client().unwrap(),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 0);
    }
}
