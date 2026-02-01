use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, StructurePosition, StructureType};
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::{InternalStructureFilter, Structure};
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;

use crate::structure::StructureError;
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::{eve_gateway_api_client, AppState};
use crate::structure::error::Result;

/// List Structures
/// 
/// - Alternative route: `/latest/internal/structures`
/// - Alternative route: `/v1/internal/structures`
/// 
/// ---
/// 
/// Lists all available structures
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    path = "/structures",
    tag = "Internal",
    params(InternalStructureFilter),
    responses(
        (
            body = Vec<Structure>,
            description = "All structures that match the filter",
            status = OK,
        ),
        (
            description = "No results for your request",
            status = NO_CONTENT
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity:      ExtractIdentity,
    State(state):  State<AppState>,
    Query(filter): Query<InternalStructureFilter>,
) -> Result<impl IntoResponse> {
    match identity.service_name.as_ref() {
        "SF_MARKET_WORKER" => (),
        _ => {
            return Ok((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "UNAUTHORIZED",
                    "description": "Not all headers are set"
                }))
            ).into_response())
        }
    }

    let data = list(
            &state.pool,
            &eve_gateway_api_client()?,
            filter,
        )
        .await?;

    if data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(data),
            )
            .into_response()
        )
    }
}

async fn list(
    pool:                   &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    filter:                 InternalStructureFilter,
) -> Result<HashMap<CharacterId, Vec<Structure>>> {
    let structures = sqlx::query!(r#"
            SELECT
                id,
                type_id,
                structure_id,
                name AS "structure_name",
                services,
                rigs,
                system_id,
                x,
                y,
                z,
                owner
            FROM structure
            WHERE ($1::INTEGER = ANY(services))
            ORDER BY structure.name
        "#,
            filter.service_id.map(|x| *x),
        )
        .fetch_all(pool)
        .await
        .map_err(|e| StructureError::ListStructures(e))?
        .into_iter()
        .collect::<Vec<_>>();

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

    let mut structure_response = HashMap::new();
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

        let structure_result = Structure {
            id:                     structure.id.into(),
            name:                   structure.structure_name,
            structure_id:           structure.structure_id,
            structure_type:         StructureType::from(structure_item.type_id),
            system:                 system.clone(),
            item:                   structure_item.clone(),
            rigs:                   rigs,
            services:               services,
            taxes:                  HashMap::new(),
            position:               StructurePosition {
                                        x: structure.x,
                                        y: structure.y,
                                        z: structure.z
                                    },

            installable_rigs:       None,
            installable_services:   None,
        };

        structure_response
            .entry(structure.owner.into())
            .and_modify(|x: &mut Vec<Structure>| x.push(structure_result.clone()))
            .or_insert(vec![structure_result]);
    }

    Ok(structure_response)
}
