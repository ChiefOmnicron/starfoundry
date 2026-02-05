use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_industry::Structure;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::industry_hub::error::{Result, IndustryHubError};
use crate::industry_hub::IndustryHubUuid;
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
                name
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
                share_type AS "share_type!: ShareType",
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
        id:         industry_hub.id.into(),
        name:       industry_hub.name,
        structures: structures,
        shares:     shares,
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

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "019a462f-c4f1-7513-93ce-0362741dacbf",
        "name": "My cool structure group",
        "structures": [{
            "id": "019a462f-dd0a-7d07-9516-df377ae11395",
            "name": "1DQ1-A - RIP",
            "structure_id": 1337,
            "system": {
                "constellation_id": 20000696,
                "constellation_name": "O-EIMK",
                "region_id": 10000060,
                "region_name": "Delve",
                "system_id": 30004759,
                "system_name": "1DQ1-A",
                "security": -0.38578233,
                "security_group": "NULLSEC",
            },
            "item": {
                "base_price": null,
                "category_id": 65,
                "group_id": 1657,
                "meta_group_id": 1,
                "name": "Keepstar",
                "repackaged": null,
                "type_id": 35834,
                "volume": 800000
            },
            "rigs": [],
            "service": [{
                "base_price": null,
                "category_id": 66,
                "group_id": 1321,
                "meta_group_id": 54,
                "name": "Standup Market Hub I",
                "repackaged": null,
                "type_id": 35892,
                "volume": 32000
            }]
        }]
    })
)]
pub struct IndustryHub {
    pub id:         IndustryHubUuid,
    pub name:       String,
    pub structures: Vec<Structure>,
    pub shares:     Vec<IndustryHubShare>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct IndustryHubShare {
    pub name:       String,
    /// either a character id, corporation id or alliance id
    pub share_id:   i32,
    pub share_type: ShareType,
}

#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type, Deserialize, Serialize, ToSchema,
)]
#[sqlx(type_name = "SHARE_TYPE")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ShareType {
    Character,
    Corporation,
    Alliance,
}
