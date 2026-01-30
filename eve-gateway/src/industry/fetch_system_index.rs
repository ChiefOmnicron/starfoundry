use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::SystemIndex;
use starfoundry_lib_types::SystemId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::Result;
use crate::industry::service::fetch_system_index;
use crate::state::AppState;

/// Fetch System Index
/// 
/// - Alternative route: `/latest/industry/system-index/{SystemId}`
/// - Alternative route: `/v1/industry/system-index/{SystemId}`
/// 
/// ---
/// 
/// Loads all open orders from a character
/// 
#[utoipa::path(
    get,
    path = "/system-index/{SystemId}",
    tag = "Industry",
    responses(
        (
            body = SystemIndex,
            description = "Index for the system",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):    State<AppState>,
    Path(system_id): Path<SystemId>,
) -> Result<impl IntoResponse> {
    let system_index = fetch_system_index(
            &state.postgres,
            system_id,
        )
        .await?;

    if system_index.is_none() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(system_index),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(system_index),
            )
            .into_response()
        )
    }
}
