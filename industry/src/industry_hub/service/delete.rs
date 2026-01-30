use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::industry_hub::IndustryHubUuid;
use crate::industry_hub::error::{Result, IndustryHubError};

pub async fn delete(
    pool:            &PgPool,
    character_id:    CharacterId,
    industry_hub_id: IndustryHubUuid,
) -> Result<IndustryHubUuid> {
    sqlx::query!("
        DELETE FROM industry_hub
        WHERE owner = $1
            AND id = $2
        RETURNING id
    ",
        *character_id,
        *industry_hub_id,
    )
    .fetch_one(pool)
    .await
    .map(|x| IndustryHubUuid::new(x.id))
    .map_err(|e| IndustryHubError::DeleteIndustryHub(e, industry_hub_id).into())
}

#[cfg(test)]
mod delete_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use std::str::FromStr;
    use uuid::Uuid;

    #[sqlx::test(
        fixtures(
            path="../fixtures",
            scripts("base")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let result = super::delete(
                &pool,
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-100000000001").unwrap().into(),
            )
            .await;
        assert!(result.is_ok());
    }
}
