use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::ProjectGroupUuid;

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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct UpdateProjectGroup {
    pub name:        String,
    pub description: Option<String>,
}

impl UpdateProjectGroup {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(ProjectGroupError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(ProjectGroupError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        match &self.description {
            Some(x) => {
                if x.len() >= 10_000 {
                    return Err(ProjectGroupError::ValidationError("Field 'description' is too long, max length: 10_000".into()));
                }
                Some(x)
            },
            None => None,
        };

        Ok(true)
    }
}

#[cfg(test)]
mod update_project_group_test {
    use sqlx::PgPool;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::project_group::error::ProjectGroupError;
    use crate::project_group::service::UpdateProjectGroup;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
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
