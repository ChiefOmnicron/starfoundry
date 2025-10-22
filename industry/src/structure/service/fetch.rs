use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item, StructureRigResponse, System};
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::structure::{StructureError, StructureUuid};
use crate::structure::error::Result;

// TODO: Permission check
pub async fn fetch(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    structure_uuid:         StructureUuid,
) -> Result<Option<Structure>> {
    let structure = sqlx::query!(r#"
            SELECT
                id,
                type_id,
                structure_id,
                name            AS "structure_name",
                services,
                rigs,
                system_id
            FROM structure
            WHERE
                owner = $1 AND
                structure.id = $2
            ORDER BY structure.name
        "#,
            *character_id,
            *structure_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| StructureError::FetchStructure(e, structure_uuid))?;

    let structure = if let Some(x) = structure {
        x
    } else {
        tracing::debug!("Couldn't find structure {}", structure_uuid);
        return Ok(None);
    };

    let structure_item = if let Some(x) = eve_gateway_api_client.fetch_item(structure.type_id.into()).await? {
        x
    } else {
        tracing::debug!("Couldn't find structure type {}", structure.type_id);
        return Ok(None);
    };
    let system = if let Some(x) = eve_gateway_api_client.fetch_system(structure.system_id.into()).await? {
        x
    } else {
        tracing::debug!("Couldn't find system {}", structure.system_id);
        return Ok(None);
    };

    let mut rigs = Vec::new();
    for rig in structure.rigs {
        if let Ok(Some(x)) = eve_gateway_api_client.fetch_rig(rig.into()).await {
            rigs.push(x);
        } else {
            // silently ignore services that weren't found
            tracing::debug!("Couldn't find rig {}", rig);
            continue;
        }
    }

    let mut services = Vec::new();
    for service in structure.services {
        if let Ok(Some(x)) = eve_gateway_api_client.fetch_item(service.into()).await {
            services.push(x);
        } else {
            // silently ignore services that weren't found
            tracing::debug!("Couldn't find service {}", service);
            continue;
        }
    }

    let structure = Structure {
        id:           structure.id.into(),
        name:         structure.structure_name,
        structure_id: structure.structure_id,
        system:       system,
        structure:    structure_item,
        rigs:         rigs,
        services:     services,
    };

    Ok(Some(structure))
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
            scripts("DELETE_AFTER_NEW_MS", "base"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let response = super::fetch(
                &pool,
                &eve_gateway_api_client().unwrap(),
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap().into(),
            )
            .await
            .unwrap();

        let response = response.unwrap();
        assert_eq!(response.name, "Some Test Structure".to_string());
    }

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("DELETE_AFTER_NEW_MS", "base"),
        ),
        migrator = "crate::test_util::MIGRATOR",
    )]
    async fn no_entry_with_default_uuid(
        pool: PgPool,
    ) {
        let response = super::fetch(
                &pool,
                &eve_gateway_api_client().unwrap(),
                CharacterId(1),
                Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap().into(),
            )
            .await;

        assert!(response.unwrap().is_none());
    }
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "id": "15bd47e3-6b38-4cc1-887b-94924fff30a1",
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
        "structure_type": {
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
    })
)]
pub struct Structure {
    /// Internal id of the structure
    pub id:                StructureUuid,
    /// EVE Id of the structure
    pub structure_id:      i64,
    /// Name of the structure
    pub name:              String,
    /// Location of the structure
    pub system:            System,
    /// Type information
    pub structure:         Item,
    /// List of all rigs that are in the structure
    pub rigs:              Vec<StructureRigResponse>,
    /// Id of the structure in-game
    pub services:          Vec<Item>,
}
