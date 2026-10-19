use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use reqwest::StatusCode;
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::{fetch_item, fetch_item_bulk};
use crate::state::AppState;
use crate::structure::StructureServiceResponse;
use crate::structure::error::{Result, StructureError};

/// List structure services
///
/// - Alternative route: `/latest/structures/{TypeId}/services`
/// - Alternative route: `/v1/structures/{TypeId}/services`
///
/// ---
///
/// Returns all services that can be installed in the structure
#[utoipa::path(
    get,
    path = "/{TypeId}/services",
    tag = "Structures",
    params(
        TypeId,
    ),
    responses(
        (
            body = StructureServiceResponse,
            description = "General information about a rig",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(structure_type_id): Path<TypeId>,
) -> Result<impl IntoResponse> {
    let entry =
        list_structure_services(&state.postgres, structure_type_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub(crate) async fn list_structure_services(
    pool: &PgPool,
    structure_type_id: TypeId,
) -> Result<Option<StructureServiceResponse>> {
    // Amarr and Jita
    if structure_type_id == TypeId(46767) || structure_type_id == TypeId(52678)
    {
        return fetch_item(pool, 35892.into())
            .await
            .map(|x| {
                let services = if let Some(y) = x {
                    vec![y]
                } else {
                    Vec::new()
                };

                Some(StructureServiceResponse {
                    services: services,
                    slots:    1,
                })
            })
            .map_err(Into::into)
    }

    let services = sqlx::query!(
        r#"
            SELECT
                service_type_ids,
                service_slots
            FROM structure_service
            WHERE structure_type_id = $1
        "#,
        *structure_type_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        StructureError::FetchStructureServices(e, structure_type_id)
    })?;

    let services = if let Some(x) = services {
        x
    } else {
        return Ok(None);
    };

    let type_ids = services
        .service_type_ids
        .into_iter()
        .map(|x| x.into())
        .collect::<Vec<_>>();

    let items = fetch_item_bulk(pool, type_ids).await?;

    Ok(Some(StructureServiceResponse {
        services: items,
        slots:    services.service_slots,
    }))
}
