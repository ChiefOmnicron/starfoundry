use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::industry_hub::error::Result;
use crate::industry_hub::error::IndustryHubError;
use crate::industry_hub::IndustryHubUuid;

pub async fn create(
    pool:         &PgPool,
    character_id: CharacterId,
    info:         CreateIndustryHub,
) -> Result<IndustryHubUuid> {
    info.valid()?;

    sqlx::query!("
            INSERT INTO industry_hub
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
        .map(|x| IndustryHubUuid::new(x.id))
        .map_err(|e| IndustryHubError::CreateIndustryHub(e))
}

#[cfg(test)]
mod create_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;

    use crate::industry_hub::error::IndustryHubError;
    use super::CreateIndustryHub;

    #[sqlx::test]
    async fn no_name(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateIndustryHub {
                    name: String::new(),
                }
            )
            .await;
        assert!(result.is_err());
        assert!(matches!(result, Err(IndustryHubError::ValidationError(_))));
    }

    #[sqlx::test]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::create(
                &pool,
                CharacterId(1),
                CreateIndustryHub {
                    name: String::from("My cool industry hub"),
                }
            )
            .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(r#"
                    SELECT
                        name
                    FROM industry_hub
                    WHERE id = $1
                "#,
                *result.unwrap(),
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My cool industry hub");
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool industry hub"
    })
)]
pub struct CreateIndustryHub {
    /// Name of the industry hub
    pub name: String,
}

impl CreateIndustryHub {
    pub fn valid(&self) -> Result<bool> {
        if self.name.len() <= 100 {
            if self.name.trim().is_empty() {
                return Err(IndustryHubError::ValidationError("Field 'name' must be set".into()));
            }
        } else {
            return Err(IndustryHubError::ValidationError("Field 'name' is too long, max length: 100".into()));
        };

        Ok(true)
    }
}
