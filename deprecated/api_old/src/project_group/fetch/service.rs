use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::project_group::error::{Error, Result};
use crate::project_group::fetch::project_group::ProjectGroupFetch;
use crate::project_group::ProjectGroupUuid;
use crate::project_group::service::{fetch_default_blacklist, fetch_default_markets, fetch_members};

pub async fn fetch(
    pool:               &PgPool,
    character_id:       CharacterId,
    project_group_uuid: ProjectGroupUuid,
) -> Result<ProjectGroupFetch> {
    let entry = sqlx::query!(
        "
            SELECT
                id,
                name,
                description,
                owner = $1 AS is_owner,
                (
                    SELECT COUNT(*)
                    FROM project_group_member
                    WHERE group_id = $2
                ) AS members,
                (
                    SELECT COUNT(*)
                    FROM project
                    WHERE project_group_id = $2
                ) AS projects
            FROM project_group pg
            WHERE pg.id = $2
        ",
            *character_id,
            *project_group_uuid,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| Error::FetchGroup(e, project_group_uuid).into())?;

    let project_group = ProjectGroupFetch {
        id:            entry.id,
        name:          entry.name,
        member_count:  entry.members.unwrap_or(1),
        project_count: entry.projects.unwrap_or(0),
        is_owner:      entry.is_owner.unwrap_or_default(),
        description:   entry.description,

        default_blacklist: fetch_default_blacklist(pool, project_group_uuid).await?,
        default_market:    fetch_default_markets(pool, character_id, project_group_uuid).await?,
        members:           fetch_members(pool, project_group_uuid).await?,
    };
    Ok(project_group)
}

#[cfg(test)]
mod fetch_project_group_test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("fetch"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        connection: PoolConnection<Postgres>,
    ) {
        let response = super::fetch(
                &pool,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        assert_eq!(response.is_owner, true);
        assert_eq!(response.name, "First".to_string());
        assert_eq!(response.description, Some("Description".into()));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("fetch"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn no_entry_with_default_uuid(
        pool: PgPool,
    ) {
        let response = super::fetch(
                &pool,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await;

        assert!(response.is_err());
    }
}
