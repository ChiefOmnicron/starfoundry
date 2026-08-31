use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::{SystemIndex, TimeSpanQuery};
use starfoundry_lib_types::SystemId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::Result;
use crate::industry::service::list_system_index;
use crate::state::AppState;

/// Fetch System Index
/// 
/// - Alternative route: `/latest/industry/system-index/{SystemId}/history`
/// - Alternative route: `/v1/industry/system-index/{SystemId}/history`
/// 
/// ---
/// 
/// Loads all open orders from a character
/// 
#[utoipa::path(
    get,
    path = "/system-index/{SystemId}/history",
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
    State(state):       State<AppState>,
    Path(system_id):    Path<SystemId>,
    Query(time_span):   Query<TimeSpanQuery>,
) -> Result<impl IntoResponse> {
    let system_index = list_system_index(
            &state.postgres,
            system_id,
            time_span,
        )
        .await?;

    if system_index.is_empty() {
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
