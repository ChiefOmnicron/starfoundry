use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::industry_hub::IndustryHubUuid;
use crate::industry_hub::error::{Result, IndustryHubError};

pub async fn update(
    pool:                &PgPool,
    character_id:        CharacterId,
    industry_hub_uuid:   IndustryHubUuid,
    industry_hub:        UpdateIndustryHub,
) -> Result<()> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(IndustryHubError::BeginTransactionError)?;

    sqlx::query!("
            UPDATE industry_hub
            SET
                name = $3
            WHERE
                id = $1 AND
                owner = $2
        ",
            *industry_hub_uuid,
            *character_id,
            industry_hub.name,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| IndustryHubError::UpdateIndustryHub(e, industry_hub_uuid))?;

    sqlx::query!("
            DELETE FROM industry_hub_structure
            WHERE industry_hub_id = $1
        ",
            *industry_hub_uuid,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| IndustryHubError::UpdateIndustryHub(e, industry_hub_uuid))?;

    sqlx::query!("
            INSERT INTO industry_hub_structure
            (
                industry_hub_id,
                structure_id
            )
            SELECT $1, * FROM UNNEST($2::UUID[])
        ",
            *industry_hub_uuid,
            &industry_hub.structures,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| IndustryHubError::UpdateIndustryHub(e, industry_hub_uuid))?;

    transaction
        .commit()
        .await
        .map(drop)
        .map_err(IndustryHubError::CommitTransactionError)
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::industry_hub::service::UpdateIndustryHub;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path_all(
        pool: PgPool,
    ) {
        let result = super::update(
            &pool,
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-100000000001").unwrap().into(),
            UpdateIndustryHub {
                name: "My structure group".into(),
                structures: vec![
                    Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap()
                ],
            }
        )
        .await;
        assert!(result.is_ok());

        let entry = sqlx::query!(r#"
                    SELECT
                        name
                    FROM industry_hub
                    WHERE id = '00000000-0000-0000-0000-100000000001'
                "#,
            )
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(entry.name, "My structure group");

        let entries = sqlx::query!(r#"
                    SELECT structure_id
                    FROM industry_hub_structure
                    WHERE industry_hub_id = '00000000-0000-0000-0000-100000000001'
                "#,
            )
            .fetch_all(&pool)
            .await
            .unwrap()
            .into_iter()
            .map(|x| x.structure_id)
            .collect::<Vec<_>>();
        assert_eq!(entries.len(), 1);
    }
}


#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "name": "My cool structure group",
        "structures": [
            "019a462f-dd0a-7d07-9516-df377ae11395",
            "019aa90f-4e83-71b3-ad36-2844f1d2701a"
        ]
    })
)]
pub struct UpdateIndustryHub {
    pub name:       String,
    pub structures: Vec<Uuid>,
}
