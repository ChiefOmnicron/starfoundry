use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::Reprocessing;
use crate::item::error::{ItemError, Result};
use crate::state::AppState;

/// Fetch Reprocessing
///
/// - Alternative route: `/latest/items/{TypeId}/reprocessing`
/// - Alternative route: `/v1/items/{TypeId}/reprocessing`
///
/// ---
///
/// Resolves reprocessing information about an item
#[utoipa::path(
    get,
    path = "/{TypeId}/reprocessing",
    tag = "Items",
    params(
        TypeId,
    ),
    responses(
        (
            body = Vec<Reprocessing>,
            description = "Information about an item",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(type_id): Path<TypeId>,
) -> Result<impl IntoResponse> {
    let entry = fetch_reprocessing(&state.postgres, type_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub async fn fetch_reprocessing(
    pool: &PgPool,
    type_id: TypeId,
) -> Result<Vec<Reprocessing>> {
    let entries = sqlx::query!(
        r#"
            SELECT
                type_id,
                material_type_id,
                quantity
            FROM item_reprocessing
            WHERE type_id = $1
        "#,
        *type_id,
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ItemError::FetchReprocessing(e, type_id))?
    .into_iter()
    .map(|x| {
        Reprocessing {
            type_id:  x
                .material_type_id
                .into(),
            quantity: x.quantity,
        }
    })
    .collect::<Vec<_>>();

    Ok(entries)
}
