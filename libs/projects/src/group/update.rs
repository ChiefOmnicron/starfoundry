use sqlx::PgPool;

use crate::{Error, ProjectGroupUuid, UpdateProjectGroup, Result};

#[deprecated]
pub async fn update(
    pool:     &PgPool,
    group_id: ProjectGroupUuid,
    info:     UpdateProjectGroup,
) -> Result<()> {
    info.valid()?;

    sqlx::query!("
        UPDATE project_group
        SET
            name = $2,
            description = $3
        WHERE id = $1
    ",
        *group_id,
        info.name,
        info.description,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| Error::UpdateGroup(e, group_id).into())
}

#[cfg(test)]
mod update_project_group_test {
    use std::str::FromStr;

    use sqlx::PgPool;
    use uuid::Uuid;

    use crate::{UpdateProjectGroup, Error};

    #[sqlx::test(
        fixtures("update"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn no_body(
        pool: PgPool,
    ) {
        let result = super::update(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
                UpdateProjectGroup {
                    name:        String::new(),
                    description: None,
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::ValidationError(_))));
    }

    #[sqlx::test(
        fixtures("update"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn missing_name(
        pool: PgPool,
    ) {
        let result = super::update(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
                UpdateProjectGroup {
                    name:        String::new(),
                    description: Some(String::from("Test description")),
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(Error::ValidationError(_))));
    }

    #[sqlx::test(
        fixtures("update"),
        migrator = "crate::test_util::MIGRATOR"
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::update(
                &pool,
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
                UpdateProjectGroup {
                    name:        String::from("My shared projects"),
                    description: Some(String::from("My cool description")),
                }
            )
            .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(
                "SELECT * FROM project_group WHERE id = '00000000-0000-0000-0000-000000000001'",
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My shared projects");
        assert_eq!(entry.description.unwrap(), "My cool description");
    }
}
