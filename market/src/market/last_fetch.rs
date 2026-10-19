use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::PgPool;
use starfoundry_lib_types::StructureId;
use utoipa::ToSchema;

use crate::AppState;
use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::{MarketError, Result};

/// Last Fetch
///
/// - Alternative route: `/latest/markets/{StructureId}/last-fetch`
/// - Alternative route: `/v1/markets/{StructureId}/last-fetch`
///
/// ---
///
/// Returns the timestamp when the last successful fetch of the market was
/// performed
#[utoipa::path(
    post,
    path = "/{StructureId}/last-fetch",
    tag = "Markets",
    responses(
        (
            body = LastFetchResponse,
            description = "No markets have the requested amount available",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
#[axum::debug_handler]
pub async fn api(
    State(state): State<AppState>,
    Path(structure_id): Path<StructureId>,
) -> Result<impl IntoResponse> {
    let finished_at = last_fetched(&state.postgres, &structure_id).await?;

    Ok((
        StatusCode::OK,
        Json(LastFetchResponse {
            finished_at,
        }),
    )
        .into_response())
}

pub async fn last_fetched(
    pool: &PgPool,
    structure_id: &StructureId,
) -> Result<NaiveDateTime> {
    let entry = sqlx::query!(
        "
            SELECT finished_at
            FROM worker_queue
            WHERE (additional_data ->> 'structure_id')::BIGINT = $1
            AND (task = 'LATEST_NPC' OR task = 'LATEST_PLAYER')
            AND status = 'DONE'
            ORDER BY finished_at DESC
            LIMIT 1
        ",
        **structure_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(MarketError::LatestFetch)?
    .ok_or(MarketError::NotFound(*structure_id))?;

    if let Some(x) = entry.finished_at {
        Ok(x.naive_utc())
    } else {
        Err(MarketError::NotFound(*structure_id))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LastFetchResponse {
    finished_at: NaiveDateTime,
}
