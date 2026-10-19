use std::collections::HashMap;

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_eve_gateway::EveGatewayApiClient;
use starfoundry_eve_gateway::structure::{StructurePosition, StructureType};
use starfoundry_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::IntoParams;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::structure::error::Result;
use crate::structure::{Structure, StructureError};
use crate::{AppState, StructureUuid, eve_gateway_api_client};

/// Fetch Structure
///
/// - Alternative route: `/latest/structures/{StructureUuid}`
/// - Alternative route: `/v1/structures/{StructureUuid}`
///
/// ---
///
/// Fetches information about a structure
///
/// ## Security
/// - authenticated
/// - structure:read
#[utoipa::path(
    get,
    path = "/{StructureUuid}",
    tag = "Structures",
    params(
        StructureUuid,
        FetchStructureQuery,
    ),
    responses(
        (
            body = Structure,
            description = "Information about the structure",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity: ExtractIdentity,
    State(state): State<AppState>,
    Path(structure_uuid): Path<StructureUuid>,
    Query(options): Query<FetchStructureQuery>,
) -> Result<impl IntoResponse> {
    let entry = fetch(
        &state.postgres,
        &eve_gateway_api_client()?,
        identity.character_id,
        structure_uuid,
        options,
    )
    .await?;

    if let Some(x) = entry {
        Ok((StatusCode::OK, Json(x)).into_response())
    } else {
        Ok((StatusCode::NO_CONTENT, ()).into_response())
    }
}

pub async fn fetch(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id: CharacterId,
    structure_uuid: StructureUuid,
    options: FetchStructureQuery,
) -> Result<Option<Structure>> {
    let result = fetch_bulk(
        pool,
        eve_gateway_api_client,
        character_id,
        vec![structure_uuid],
        options,
    )
    .await?;

    if let Some(x) = result.first() {
        Ok(Some(x.clone()))
    } else {
        Ok(None)
    }
}

pub async fn fetch_bulk(
    pool: &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    character_id: CharacterId,
    structure_uuids: Vec<StructureUuid>,
    options: FetchStructureQuery,
) -> Result<Vec<Structure>> {
    let structures = sqlx::query!(
        r#"
            SELECT
                id,
                type_id,
                structure_id,
                name            AS "structure_name",
                services,
                rigs,
                system_id,
                x,
                y,
                z
            FROM structure
            WHERE
                (owner = $1 OR owner = 0) AND
                id = ANY($2)
        "#,
        *character_id,
        &structure_uuids
            .iter()
            .map(|x| **x)
            .collect::<Vec<_>>(),
    )
    .fetch_all(pool)
    .await
    .map_err(|e| StructureError::FetchStructures(e, structure_uuids.clone()))?;

    if structures.is_empty() {
        return Ok(Vec::new());
    }

    let mut result = Vec::new();

    let type_ids = structures
        .iter()
        .map(|x| x.type_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    let system_ids = structures
        .iter()
        .map(|x| x.system_id)
        .map(Into::into)
        .collect::<Vec<_>>();

    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();
    let systems = eve_gateway_api_client
        .fetch_system_bulk(system_ids)
        .await?
        .into_iter()
        .map(|x| (x.system_id, x))
        .collect::<HashMap<_, _>>();

    for structure in structures {
        let taxes = sqlx::query!(
            "
                SELECT
                    service_type_id,
                    tax
                FROM structure_tax
                WHERE structure_id = $1
            ",
            structure.id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| {
            StructureError::FetchStructureTax(
                e,
                structure
                    .id
                    .into(),
            )
        })?
        .into_iter()
        .map(|x| {
            (
                x.service_type_id
                    .into(),
                x.tax,
            )
        })
        .collect::<HashMap<_, _>>();

        let item = if let Some(x) = items.get(
            &structure
                .type_id
                .into(),
        ) {
            x.clone()
        } else {
            continue;
        };
        let system = if let Some(x) = systems.get(
            &structure
                .system_id
                .into(),
        ) {
            x.clone()
        } else {
            continue;
        };

        let mut rigs = Vec::new();
        for rig in structure.rigs {
            if let Ok(Some(x)) = eve_gateway_api_client
                .fetch_rig(rig.into())
                .await
            {
                rigs.push(x);
            } else {
                // silently ignore services that weren't found
                tracing::debug!("Couldn't find rig {}", rig);
                continue;
            }
        }

        let mut services = Vec::new();
        for service in structure.services {
            if let Ok(Some(x)) = eve_gateway_api_client
                .fetch_item(service.into())
                .await
            {
                services.push(x);
            } else {
                // silently ignore services that weren't found
                tracing::debug!("Couldn't find service {}", service);
                continue;
            }
        }

        let mut installable_rigs = None;
        let mut installable_services = None;
        if let Some(true) = options.include_installable {
            if let Ok(x) = eve_gateway_api_client
                .list_structure_rigs(
                    structure
                        .type_id
                        .into(),
                )
                .await
            {
                installable_rigs = Some(x);
            } else {
                // silently ignore services that weren't found
                tracing::debug!(
                    "Couldn't list rigs for type_id {}",
                    structure.type_id
                );
            }

            if let Ok(x) = eve_gateway_api_client
                .list_structure_services(
                    structure
                        .type_id
                        .into(),
                )
                .await
            {
                installable_services = Some(x);
            } else {
                // silently ignore services that weren't found
                tracing::debug!(
                    "Couldn't list services for type_id {}",
                    structure.type_id
                );
            }
        }

        let structure = Structure {
            id:             structure
                .id
                .into(),
            name:           structure.structure_name,
            structure_id:   structure.structure_id,
            structure_type: StructureType::from(item.type_id),
            system:         system,
            item:           item,
            rigs:           rigs,
            services:       services,
            taxes:          taxes,
            position:       StructurePosition {
                x: structure.x,
                y: structure.y,
                z: structure.z,
            },

            installable_rigs:     installable_rigs,
            installable_services: installable_services,
        };
        result.push(structure);
    }

    Ok(result)
}

#[derive(Debug, Default, Deserialize, Serialize, IntoParams)]
pub struct FetchStructureQuery {
    #[serde(default)]
    pub include_installable: Option<bool>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use axum::body::Body;
    use axum::extract::Request;
    use axum::http::StatusCode;
    use axum::http::header::HOST;
    use sqlx::PgPool;
    use starfoundry_gateway::{
        HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SERVICE
    };
    use starfoundry_lib_types::CharacterId;
    use uuid::Uuid;

    use crate::eve_gateway_api_client;
    use crate::structure::fetch::FetchStructureQuery;
    use crate::structure::structure_test_routes;

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn direct_happy_path(pool: PgPool) {
        let response = super::fetch(
            &pool,
            &eve_gateway_api_client().unwrap(),
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-000000000001")
                .unwrap()
                .into(),
            FetchStructureQuery {
                include_installable: None,
            },
        )
        .await
        .unwrap();

        let response = response.unwrap();
        assert_eq!(response.name, "Some Test Structure".to_string());
    }

    #[sqlx::test(fixtures(path = "./fixtures", scripts("base"),))]
    async fn no_entry_with_default_uuid(pool: PgPool) {
        let response = super::fetch(
            &pool,
            &eve_gateway_api_client().unwrap(),
            CharacterId(1),
            Uuid::from_str("00000000-0000-0000-0000-000000000000")
                .unwrap()
                .into(),
            FetchStructureQuery {
                include_installable: None,
            },
        )
        .await;

        assert!(
            response
                .unwrap()
                .is_none()
        );
    }

    #[sqlx::test(fixtures("base"))]
    async fn happy_path(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::OK);
    }

    #[sqlx::test(fixtures("base"))]
    async fn unauthorized(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000001")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[sqlx::test(fixtures("base"))]
    async fn forbidden(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000002")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::FORBIDDEN);
    }

    #[sqlx::test(fixtures("base"))]
    async fn not_found(pool: PgPool) {
        let request = Request::builder()
            .uri("/00000000-0000-0000-0000-000000000000")
            .method("GET")
            .header(HEADER_SERVICE, "industry.test")
            .header(HEADER_CHARACTER_ID, 1)
            .header(HEADER_CORPORATION_ID, 1)
            .header(HOST, "test.starfoundry.space")
            .body(Body::empty())
            .unwrap();
        let response = structure_test_routes(pool, request).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
