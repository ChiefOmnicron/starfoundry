use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::SystemDistance;
use starfoundry_lib_types::SystemId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::system::error::Result;
use crate::system::services::list_in_range;

/// List Systems in range
///
/// - Alternative route: `/latest/systems/{SystemId}/distances`
/// - Alternative route: `/v1/systems/{SystemId}/distances`
///
/// ---
///
/// Lists all system that are in range
///
#[utoipa::path(
    get,
    path = "/{SystemId}/distances",
    tag = "System",
    params(
        SystemId,
    ),
    responses(
        (
            body = Vec<SystemDistance>,
            description = "List of all systems in range",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):       State<AppState>,
    Path(system_id):    Path<SystemId>,
) -> Result<impl IntoResponse> {
    let entry = list_in_range(
            &state.postgres,
            system_id,
        )
        .await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
