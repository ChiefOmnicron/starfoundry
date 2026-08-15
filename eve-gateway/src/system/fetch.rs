use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::System;
use starfoundry_lib_types::{SystemId, TypeId};

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::system::error::Result;
use crate::system::services::fetch_system;

/// Fetch System
/// 
/// - Alternative route: `/latest/systems/{SystemId}`
/// - Alternative route: `/v1/systems/{SystemId}`
/// 
/// ---
/// 
/// Resolves all information about a system
/// 
#[utoipa::path(
    get,
    path = "/systems/{SystemId}",
    tag = "System",
    params(
        TypeId,
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
    State(state):    State<AppState>,
    Path(system_id): Path<SystemId>,
) -> Result<impl IntoResponse> {
    let entry = fetch_system(
        &state.postgres,
        system_id,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
