use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;
use std::fmt;
use utoipa::IntoParams;

use crate::structure::error::Result;
use crate::structure::service::Structure;
use crate::structure::StructureError;

pub async fn list(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id:           CharacterId,
    filter:                 StructureFilter,
) -> Result<Vec<Structure>> {
    let structures = sqlx::query!(r#"
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
                NOT (LOWER(structure.name) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                NOT (structure.type_id = $3) IS FALSE AND
                NOT ($4::INTEGER IS NULL OR $4::INTEGER = ANY(services)) IS FALSE AND
                owner = $1
            ORDER BY structure.name
        "#,
            *character_id,
            filter.name,
            filter.structure_type_id,
            filter.service_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureError::ListStructures(e))?;

    let mut type_ids = structures
        .iter()
        .map(|x| x.type_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    let service_type_ids = structures
        .iter()
        .flat_map(|x| x.services.clone())
        .map(Into::into)
        .collect::<Vec<_>>();
    type_ids.extend_from_slice(&service_type_ids);
    type_ids.sort();
    type_ids.dedup();

    let system_ids = structures
        .iter()
        .map(|x| x.system_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    type_ids.sort();
    type_ids.dedup();

    let resolved_type_ids = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();
    let resolved_system_ids = eve_gateway_api_client
        .fetch_system_bulk(system_ids)
        .await?
        .into_iter()
        .map(|x| (x.system_id, x))
        .collect::<HashMap<_, _>>();

    let mut structure_result = Vec::new();
    for structure in structures {
        // TODO: add bulk function
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
            if let Some(x) = resolved_type_ids.get(&service.into()) {
                services.push(x.clone());
            } else {
                // silently ignore services that weren't found
                tracing::debug!("Couldn't find service {}", service);
                continue;
            }
        }

        let structure_item = if let Some(x) = resolved_type_ids.get(&structure.type_id.into()) {
            x
        } else {
            // silently ignore type_ids that weren't found
            tracing::debug!("Couldn't find type_id {}", structure.type_id);
            continue;
        };

        let system = if let Some(x) = resolved_system_ids.get(&structure.system_id.into()) {
            x
        } else {
            // silently ignore system that weren't found
            tracing::debug!("Couldn't find system {}", structure.system_id);
            continue;
        };

        let structure = Structure {
            id:           structure.id.into(),
            name:         structure.structure_name,
            structure_id: structure.structure_id,
            system:       system.clone(),
            structure:    structure_item.clone(),
            rigs:         rigs,
            services:     services,
        };
        structure_result.push(structure);
    }

    Ok(structure_result)
}

#[derive(Debug, Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct StructureFilter {
    #[serde(default)]
    #[param(
        example = json!("My cool structure"),
        required = false,
    )]
    pub name:              Option<String>,

    #[serde(default)]
    #[param(
        example = json!("35827"),
        required = false,
    )]
    pub structure_type_id: Option<i32>,

    /// [TypeId] of a structure service
    #[serde(default)]
    #[param(
        example = json!("35892"),
        required = false,
    )]
    pub service_id:        Option<i32>,
}

impl fmt::Display for StructureFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self, f)
    }
}
