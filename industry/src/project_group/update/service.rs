use sqlx::PgPool;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;
use crate::project_group::update::UpdateProjectGroup;

pub async fn update(
    pool:               &PgPool,
    project_group_uuid: ProjectGroupUuid,
    update_info:        UpdateProjectGroup,
) -> Result<()> {
    update_info.valid()?;

    sqlx::query!("
        UPDATE project_group
        SET
            name = $2,
            description = $3
        WHERE id = $1
    ",
        *project_group_uuid,
        update_info.name,
        update_info.description,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| ProjectGroupError::UpdateGroup(e, project_group_uuid).into())
}

#[cfg(test)]
mod update_project_group_test {
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::project_group::error::ProjectGroupError;
    use crate::project_group::update::UpdateProjectGroup;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
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
        assert!(matches!(result, Err(ProjectGroupError::ValidationError(_))));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
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
        assert!(matches!(result, Err(ProjectGroupError::ValidationError(_))));
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
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
