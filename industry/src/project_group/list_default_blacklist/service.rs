use sqlx::PgPool;

use crate::item::Item;
use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

pub async fn list_default_blacklist(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
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

    let mut items = Vec::new();
    for type_id in type_ids {
        // silently ignore errors
        if let Ok(Some(x)) = Item::new(&pool, type_id).await {
            items.push(x);
        } else {
            tracing::debug!("Didn't found an item for {}", type_id);
        }
    }

    Ok(items)
}

#[cfg(test)]
mod list_default_blacklist_project_group_test {
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
        let response = super::list_default_blacklist(
                &pool,
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
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let response = super::list_default_blacklist(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.len(), 0);
    }
}
