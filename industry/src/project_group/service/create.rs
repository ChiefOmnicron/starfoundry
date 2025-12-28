use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use std::str::FromStr;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::project_group::error::{ProjectGroupError, Result};
use crate::project_group::permission::ProjectGroupPermissionCode;
use crate::project_group::ProjectGroupUuid;

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    info:         CreateProjectGroup,
) -> Result<ProjectGroupUuid> {
    info.valid()?;

    let mut transaction = pool
        .begin()
        .await
        .map_err(ProjectGroupError::TransactionBeginError)?;

    // create the group
    let group_id = sqlx::query!("
            INSERT INTO project_group(
                owner,
                name
            )
            VALUES ($1, $2)
            RETURNING id
        ",
            *character_id,
            info.name,
        )
        .fetch_one(&mut *transaction)
        .await
        .map(|x| ProjectGroupUuid::new(x.id))
        .map_err(|e| ProjectGroupError::CreateGroup(e))?;

    // add the owner as member of the group
    sqlx::query!("
            INSERT INTO project_group_member(
                accepted,
                project_group_id,
                character_id,
                permission
            )
            VALUES (
                TRUE, $1, $2, $3
            )
        ",
            *group_id,
            *character_id,
            *ProjectGroupPermissionCode::Owner,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::AcceptGroupMember(e, group_id))?;

    // add the defaults
    sqlx::query!("
            INSERT INTO project_group_default_market(
                project_group_id,
                structure_id
            )
            VALUES (
                $1, $2
            )
        ",
            *group_id,
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap_or_default(), // jita
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| ProjectGroupError::AcceptGroupMember(e, group_id))?;

    transaction
        .commit()
        .await
        .map_err(ProjectGroupError::TransactionCommitError)?;
    Ok(group_id)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool group"
    })
)]
pub struct CreateProjectGroup {
    /// Maximum length 100
    pub name:        String,
}

impl CreateProjectGroup {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(ProjectGroupError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(ProjectGroupError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        Ok(true)
    }
}

#[cfg(test)]
mod create_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::project_group::error::ProjectGroupError;
    use crate::project_group::service::CreateProjectGroup;

    #[sqlx::test()]
    async fn no_body(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateProjectGroup {
                    name: String::new(),
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(ProjectGroupError::ValidationError(_))));
    }

    #[sqlx::test()]
    async fn empty_name(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateProjectGroup {
                    name: String::new(),
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(ProjectGroupError::ValidationError(_))));
    }

    #[sqlx::test()]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateProjectGroup {
                    name: String::from("My shared projects"),
                }
            )
            .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(
                "SELECT * FROM project_group WHERE id = $1",
                *result.unwrap(),
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My shared projects");
    }
}
