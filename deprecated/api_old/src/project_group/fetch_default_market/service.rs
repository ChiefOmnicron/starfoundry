use sqlx::PgPool;
use starfoundry_lib_structures::{StructureDatabase, StructureService};
use starfoundry_lib_types::CharacterId;

use crate::project_group::error::{Error, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn fetch_default_markets(
    pool:               &PgPool,
    character_id:       CharacterId,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Vec<StructureDatabase>> {
    let markets = sqlx::query!("
            SELECT structure_id
            FROM project_group_default_market
            WHERE project_group_id = $1
        ",
            *project_group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchGroupDefaults(e, project_group_uuid))?
        .into_iter()
        .map(|x| x.structure_id.into())
        .collect::<Vec<_>>();

    let mut structures = Vec::new();
    for market in markets {
        let structure = StructureService::new(market);
        // silently ignore errors
        if let Ok(Some(x)) = structure.fetch(pool, character_id).await {
            structures.push(x);
        }
    }

    Ok(structures)
}

#[cfg(test)]
mod fetch_default_markets_project_group_test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use uuid::Uuid;
    use starfoundry_lib_types::CharacterId;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("fetch", "fetch_default"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        connection: PoolConnection<Postgres>,
    ) {
        let response = super::fetch_default_markets(
                &pool,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 1);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("fetch", "fetch_default"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let response = super::fetch_default_markets(
                &pool,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 0);
    }
}
