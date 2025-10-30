use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::System;
use starfoundry_lib_types::SystemId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::universe::error::Result;
use crate::universe::services::fetch_system_bulk;

/// Fetch an item
/// 
/// - Alternative route: `/latest/universe/systems`
/// - Alternative route: `/v1/universe/systems`
/// 
/// ---
/// 
/// Resolves all information about an item
/// 
#[utoipa::path(
    post,
    path = "/systems",
    tag = "Universe",
    request_body = Vec<SystemId>,
    responses(
        (
            body = Vec<System>,
            description = "Information about the given systems",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):     State<AppState>,
    Json(system_ids): Json<Vec<SystemId>>,
) -> Result<impl IntoResponse> {
    let entry = fetch_system_bulk(
        &state.postgres,
        system_ids,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
