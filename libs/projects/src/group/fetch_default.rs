use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::{Error, ProjectGroupDefault, ProjectGroupUuid, Result};

#[deprecated]
pub async fn fetch_defaults(
    pool:       &PgPool,
    group_uuid: ProjectGroupUuid,
) -> Result<ProjectGroupDefault> {
    let blacklist: Vec<TypeId> = sqlx::query!("
            SELECT type_id
            FROM project_group_default_blacklist
            WHERE project_group_id = $1
        ",
            *group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchGroupDefaults(e, group_uuid))?
        .into_iter()
        .map(|x| x.type_id.into())
        .collect::<Vec<_>>();

    let markets = sqlx::query!("
            SELECT structure_id
            FROM project_group_default_market
            WHERE project_group_id = $1
        ",
            *group_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchGroupDefaults(e, group_uuid))?
        .into_iter()
        .map(|x| x.structure_id.into())
        .collect::<Vec<_>>();

    Ok(ProjectGroupDefault {
        blacklist,
        markets,
    })
}

#[cfg(test)]
mod fetch_defaults_project_group_test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures("fetch", "fetch_default"),
        //migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::fetch_defaults(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.blacklist.len(), 4);
        assert_eq!(response.markets.len(), 1);
    }

    #[sqlx::test(
        fixtures("fetch", "fetch_default"),
        //migrator = "crate::test_util::MIGRATOR",
    )]
    async fn default_if_entry_does_not_exist(
        pool: PgPool,
    ) {
        let response = super::fetch_defaults(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.blacklist.len(), 0);
        assert_eq!(response.markets.len(), 0);
    }
}
