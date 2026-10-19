use axum::Json;
use axum::extract::{Path, State};
use axum::response::IntoResponse;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::Item;
use crate::item::fetch_item_bulk;
use crate::state::AppState;
use crate::structure::error::{Result, StructureError};

/// Fetch structure services
///
/// - Alternative route: `/latest/structures/services/{TypeId}`
/// - Alternative route: `/v1/structures/services/{TypeId}`
///
/// ---
///
/// Returns all services that can be installed in the structure
#[utoipa::path(
    get,
    path = "/services/{TypeId}",
    tag = "Structures",
    params(
        TypeId,
    ),
    responses(
        (
            body = Vec<Item>,
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
    let entry = fetch_services(&state.postgres, structure_type_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub(crate) async fn fetch_services(
    pool: &PgPool,
    structure_type_id: TypeId,
) -> Result<StructureServiceResponse> {
    let services = sqlx::query!(
        r#"
            SELECT service_type_ids, service_slots
            FROM structure_service
            WHERE structure_type_id = $1
        "#,
        *structure_type_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        StructureError::FetchStructureServices(e, structure_type_id)
    })?;

    let type_ids = services
        .iter()
        .map(|x| {
            x.service_type_ids
                .clone()
        })
        .flat_map(|x| {
            x.into_iter()
                .map(Into::into)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let items = fetch_item_bulk(pool, type_ids).await?;

    Ok(StructureServiceResponse {
        slots:    services
            .first()
            .map(|x| x.service_slots)
            .unwrap_or_default(),
        services: items,
    })
}

#[derive(Clone, Debug, Default, Deserialize, Serialize, ToSchema)]
pub struct StructureServiceResponse {
    pub services: Vec<Item>,
    pub slots:    i32,
}
