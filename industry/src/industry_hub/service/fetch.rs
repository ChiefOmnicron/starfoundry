use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::industry_hub::{IndustryHub, IndustryHubShare, IndustryHubShareType};
use starfoundry_lib_industry::IndustryHubUuid;
use starfoundry_lib_types::CharacterId;

use crate::industry_hub::error::{Result, IndustryHubError};
use crate::structure::service::StructureFilter;

// TODO: Permission check
pub async fn fetch(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    industry_hub_uuid:      IndustryHubUuid,
) -> Result<Option<IndustryHub>> {
    let industry_hub = sqlx::query!(r#"
            SELECT
                id,
                name,
                description
            FROM industry_hub
            WHERE
                owner = $1 AND
                id = $2
        "#,
            *character_id,
            *industry_hub_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| IndustryHubError::FetchIndustryHub(e, industry_hub_uuid))?;

    let industry_hub = if let Some(x) = industry_hub {
        x
    } else {
        tracing::debug!("Couldn't find structure group {}", industry_hub_uuid);
        return Ok(None);
    };

    let structure_ids = sqlx::query!("
            SELECT
                structure_id
            FROM industry_hub_structure
            WHERE industry_hub_id = $1
        ",
            industry_hub.id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| IndustryHubError::FetchIndustryHub(e, industry_hub.id.into()))?
        .into_iter()
        .map(|x| x.structure_id.into())
        .collect();

    let shares = sqlx::query!(r#"
            SELECT
                share_id,
                share_type AS "share_type!: IndustryHubShareType",
                name
            FROM industry_hub_share
            WHERE industry_hub_id = $1
        "#,
            industry_hub.id,
        )
        .fetch_all(pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|x| IndustryHubShare {
            name: x.name,
            share_id: x.share_id,
            share_type: x.share_type,
        })
        .collect::<Vec<_>>();

    let structures = crate::structure::service::list(
            &pool,
            eve_gateway_api_client,
            character_id,
            StructureFilter {
                structure_ids: Some(structure_ids),
                ..Default::default()
            }
        )
        .await?;

    let industry_hub = IndustryHub {
        id:          industry_hub.id.into(),
        name:        industry_hub.name,
        structures:  structures,
        shares:      shares,
        description: industry_hub.description,
    };

    Ok(Some(industry_hub))
}

#[cfg(test)]
mod tests {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::eve_gateway_api_client;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::fetch(
                &pool,
                &eve_gateway_api_client().unwrap(),
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-100000000001").unwrap().into(),
            )
            .await
            .unwrap();

        let response = response.unwrap();
        assert_eq!(response.name, "IndustryHubA".to_string());
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base"),
        ),
    )]
    async fn no_entry_with_default_uuid(
        pool: PgPool,
    ) {
        let response = super::fetch(
                &pool,
                &eve_gateway_api_client().unwrap(),
                CharacterId(1),
                Uuid::default().into(),
            )
            .await;

        assert!(response.unwrap().is_none());
    }
}
