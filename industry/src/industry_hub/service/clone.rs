use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::industry_hub::error::Result;
use crate::industry_hub::error::IndustryHubError;
use crate::industry_hub::IndustryHubUuid;

pub async fn clone(
    pool:            &PgPool,
    character_id:    CharacterId,
    industry_hub_id: IndustryHubUuid,
) -> Result<IndustryHubUuid> {
    let mut transaction = pool
        .begin()
        .await
        .map_err(IndustryHubError::BeginTransactionError)?;

    let cloned_id: IndustryHubUuid = sqlx::query!("
            INSERT INTO industry_hub
            (
                owner,
                name
            )
            VALUES(
                $1,
                (SELECT name FROM industry_hub WHERE id = $2)
            )
            RETURNING id
        ",
            *character_id,
            *industry_hub_id,
        )
        .fetch_one(&mut *transaction)
        .await
        .map(|x| IndustryHubUuid::new(x.id))
        .map_err(|e| IndustryHubError::CreateIndustryHub(e))?
        .into();

    let structure_ids = sqlx::query!("
            INSERT INTO structure
            (
                owner,
                structure_id,
                system_id,
                type_id,
                rigs,
                services,
                name
            )
            SELECT
                $1 AS owner,
                structure_id,
                system_id,
                type_id,
                rigs,
                services,
                name
            FROM structure s
            RETURNING id
        ",
            *character_id,
        )
        .fetch_all(&mut *transaction)
        .await
        .map_err(|e| IndustryHubError::CloneIndustryHub(e, industry_hub_id))?
        .into_iter()
        .map(|x| x.id)
        .collect::<Vec<_>>();

    sqlx::query!("
            INSERT INTO industry_hub_structure
            (
                industry_hub_id,
                structure_id
            )
            SELECT $1, * FROM UNNEST($2::UUID[])
        ",
            *cloned_id,
            &structure_ids,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| IndustryHubError::UpdateIndustryHub(e, cloned_id))?;

    sqlx::query!("
            INSERT INTO industry_hub_clone
            (
                original_id,
                new_id
            )
            VALUES ($1, $2)
        ",
            *industry_hub_id,
            *cloned_id,
        )
        .execute(&mut *transaction)
        .await
        .map_err(|e| IndustryHubError::CreateIndustryHub(e))?;

    transaction
        .commit()
        .await
        .map_err(IndustryHubError::CommitTransactionError)?;

    Ok(cloned_id)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "SHARE_TYPE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShareType {
    Character,
    Corporation,
    Alliance,
}
