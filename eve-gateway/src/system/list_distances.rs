use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::SystemDistanceMinimal;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::system::error::Result;
use crate::system::services::list_distances;

/// Fetch Distances
///
/// - Alternative route: `/latest/systems/distances`
/// - Alternative route: `/v1/systems/distances`
///
/// ---
///
/// Lists all system distances
///
#[utoipa::path(
    get,
    path = "/distances",
    tag = "System",
    responses(
        (
            body = Vec<SystemDistanceMinimal>,
            description = "Returns a list of all systems with their ranges",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>
) -> Result<impl IntoResponse> {
    let entry = list_distances(
            &state.postgres
        )
        .await?;

    Ok(
        (
            StatusCode::OK, Json(entry)
        )
        .into_response()
    )
}
