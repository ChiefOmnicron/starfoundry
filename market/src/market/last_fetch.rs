use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_types::StructureId;
use chrono::NaiveDateTime;
use serde::Serialize;
use utoipa::ToSchema;

use crate::api_docs::{InternalServerError, NotFound};
use crate::AppState;
use crate::market::error::Result;
use crate::market::last_fetched;

/// Update Virtual
/// 
/// - Alternative route: `/latest/markets/{StructureId}/last-fetch`
/// - Alternative route: `/v1/markets/{StructureId}/last-fetch`
/// 
/// ---
/// 
/// Returns the timestamp when the last successful fetch of the market was
/// performed 
/// 
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
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureId>,
) -> Result<impl IntoResponse> {
    let finished_at = last_fetched(
            &state.postgres,
            structure_id,
        )
        .await?;

    Ok(
        (
            StatusCode::OK,
            Json(LastFetchResponse {
                finished_at,
            }),
        )
        .into_response()
    )
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LastFetchResponse {
    finished_at: NaiveDateTime,
}
