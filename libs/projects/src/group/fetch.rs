use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::{Error, ProjectGroupUuid, ProjectGroup, Result};

#[deprecated]
pub async fn fetch(
    pool:         &PgPool,
    character_id: CharacterId,
    group_id:     ProjectGroupUuid,
) -> Result<ProjectGroup> {
    sqlx::query!(
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
            *group_id,
        )
        .fetch_one(pool)
        .await
        .map(|x| {
            ProjectGroup {
                id:          x.id,
                name:        x.name,
                members:     x.members.unwrap_or(1),
                projects:    x.projects.unwrap_or(0),
                is_owner:    x.is_owner.unwrap_or_default(),

                description: x.description,
            }
        })
        .map_err(|e| Error::FetchGroup(e, group_id).into())
}

#[cfg(test)]
mod fetch_project_group_test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures("fetch"),
        //migrator = "crate::test_util::MIGRATOR",
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

        assert_eq!(response.is_owner, true);
        assert_eq!(response.name, "First".to_string());
        assert_eq!(response.description, Some("Description".into()));
    }

    #[sqlx::test(
        fixtures("fetch"),
        //migrator = "crate::test_util::MIGRATOR",
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
