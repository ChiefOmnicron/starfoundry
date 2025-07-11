use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::fetch::project_group::ProjectGroupFetch;
use crate::project_group::ProjectGroupUuid;

pub async fn fetch(
    pool:               &PgPool,
    character_id:       CharacterId,
    project_group_uuid: ProjectGroupUuid,
) -> Result<Option<ProjectGroupFetch>> {
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
        .fetch_optional(pool)
        .await
        .map_err(|e| ProjectGroupError::FetchGroup(e, project_group_uuid).into())?;

    if let Some(x) = entry {
        let project_group = ProjectGroupFetch {
            id:                x.id,
            name:              x.name,
            member_count:      x.members.unwrap_or(1),
            project_count:     x.projects.unwrap_or(0),
            is_owner:          x.is_owner.unwrap_or_default(),
            description:       x.description,

            default_blacklist: Vec::new(),//fetch_default_blacklist(pool, project_group_uuid).await?,
            default_market:    Vec::new(),//fetch_default_markets(pool, character_id, project_group_uuid).await?,
            members:           Vec::new(),//fetch_members(pool, project_group_uuid).await?,
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
    use starfoundry_libs_types::CharacterId;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("fetch"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::fetch(
                &pool,
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
