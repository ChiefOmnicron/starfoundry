use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::structure_group::StructureGroupUuid;
use crate::structure_group::error::{Result, StructureGroupError};
use crate::structure::service::{Structure, StructureFilter};

// TODO: Permission check
pub async fn fetch(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    structure_group_uuid:   StructureGroupUuid,
) -> Result<Option<StructureGroup>> {
    let structure_group = sqlx::query!(r#"
            SELECT
                id,
                name
            FROM structure_group
            WHERE
                owner = $1 AND
                id = $2
        "#,
            *character_id,
            *structure_group_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureGroupError::FetchStructureGroup(e, structure_group_uuid))?;

    let structure_group = if let Some(x) = structure_group {
        x
    } else {
        tracing::debug!("Couldn't find structure group {}", structure_group_uuid);
        return Ok(None);
    };

    let structure_ids = sqlx::query!("
            SELECT
                structure_id
            FROM structure_group_structure
            WHERE structure_group_id = $1
        ",
            structure_group.id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureGroupError::FetchStructureGroup(e, structure_group.id.into()))?
        .into_iter()
        .map(|x| x.structure_id.into())
        .collect();

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

    let structure_group = StructureGroup {
        id:         structure_group.id.into(),
        name:       structure_group.name,
        structures: structures,
    };

    Ok(Some(structure_group))
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
        assert_eq!(response.name, "StructureGroupA".to_string());
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

#[derive(Debug, Deserialize, Serialize, ToSchema)]
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
pub struct StructureGroup {
    pub id:         StructureGroupUuid,
    pub name:       String,
    pub structures: Vec<Structure>,
}
