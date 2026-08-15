use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::System;
use starfoundry_lib_types::SystemId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::system::error::Result;
use crate::system::services::fetch_distance;

/// Fetch Distance
///
/// - Alternative route: `/latest/systems/{SystemId}/distances/{SystemId}`
/// - Alternative route: `/v1/systems/{SystemId}/distances/{SystemId}`
///
/// ---
///
/// Fetches the distance in LY between the given systems
///
#[utoipa::path(
    get,
    path = "/systems/{SystemId}/distances/{SystemId}",
    tag = "System",
    params(
        SystemId,
        SystemId,
    ),
    responses(
        (
            body = System,
            description = "Information about a system",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path((start_system_id, end_system_id)): Path<(SystemId, SystemId)>,
) -> Result<impl IntoResponse> {
    let entry = fetch_distance(&state.postgres, start_system_id, end_system_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}
