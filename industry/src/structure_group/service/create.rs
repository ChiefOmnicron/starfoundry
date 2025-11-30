use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::structure_group::error::Result;
use crate::structure_group::error::StructureGroupError;
use crate::structure_group::StructureGroupUuid;

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    info:         CreateStructureGroup,
) -> Result<StructureGroupUuid> {
    info.valid()?;

    sqlx::query!("
            INSERT INTO structure_group
            (
                owner,
                name
            )
            VALUES($1, $2)
            RETURNING id
        ",
            *character_id,
            info.name,
        )
        .fetch_one(pool)
        .await
        .map(|x| StructureGroupUuid::new(x.id))
        .map_err(|e| StructureGroupError::CreateStructureGroup(e))
}

#[cfg(test)]
mod create_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::structure_group::error::StructureGroupError;
    use super::CreateStructureGroup;

    #[sqlx::test]
    async fn no_name(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateStructureGroup {
                    name:              String::new(),
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(StructureGroupError::ValidationError(_))));
    }

    #[sqlx::test]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateStructureGroup {
                    name: String::from("My cool structure group"),
                }
            )
            .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(r#"
                    SELECT
                        name
                    FROM structure_group
                    WHERE id = $1
                "#,
                *result.unwrap(),
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My cool structure group");
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool structure"
    })
)]
pub struct CreateStructureGroup {
    /// Name of the structure
    pub name: String,
}

impl CreateStructureGroup {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(StructureGroupError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(StructureGroupError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        Ok(true)
    }
}
