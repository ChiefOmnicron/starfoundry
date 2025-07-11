use sqlx::PgPool;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::structure::Structure;

pub async fn list_default_market(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
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
        if let Some(x) = Structure::new(pool, market).await? {
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
    use std::str::FromStr;

    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base", "list_default"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::list_default_market(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 1);
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base", "list_default"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let response = super::list_default_market(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 0);
    }
}
