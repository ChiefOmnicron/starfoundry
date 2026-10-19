use std::collections::HashMap;

use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::Reprocessing;
use crate::item::error::{ItemError, Result};
use crate::state::AppState;

/// Bulk Fetch Reprocessing
///
/// - Alternative route: `/latest/items/reprocessing`
/// - Alternative route: `/v1/items/reprocessing`
///
/// ---
///
/// Bulk information about reprocessing
#[utoipa::path(
    post,
    path = "/reprocessing",
    tag = "Items",
    request_body = Vec<TypeId>,
    responses(
        (
            body = HashMap<TypeId, Vec<Reprocessing>>,
            description = "Reprocessing Information about an item",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Json(type_ids): Json<Vec<TypeId>>,
) -> Result<impl IntoResponse> {
    let entry = fetch_bulk_reprocessing(&state.postgres, type_ids).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub async fn fetch_bulk_reprocessing(
    pool: &PgPool,
    type_ids: Vec<TypeId>,
) -> Result<HashMap<TypeId, Vec<Reprocessing>>> {
    let entries = sqlx::query!(
        r#"
            SELECT
                type_id,
                material_type_id,
                quantity
            FROM item_reprocessing
            WHERE type_id = ANY($1)
        "#,
        &*type_ids
            .into_iter()
            .map(|x| *x)
            .collect::<Vec<_>>(),
    )
    .fetch_all(pool)
    .await
    .map_err(|e| ItemError::FetchBulkReprocessing(e))?;

    let mut result = HashMap::new();
    for entry in entries {
        let reprocessing = Reprocessing {
            type_id:  entry
                .material_type_id
                .into(),
            quantity: entry.quantity,
        };

        result
            .entry(
                entry
                    .type_id
                    .into(),
            )
            .and_modify(|x: &mut Vec<Reprocessing>| {
                x.push(reprocessing.clone())
            })
            .or_insert(vec![reprocessing]);
    }

    Ok(result)
}
