use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{System, SystemSearchQuery};

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::system::error::Result;
use crate::system::services::list;

/// Fetch System
/// 
/// - Alternative route: `/latest/systems`
/// - Alternative route: `/v1/systems`
/// 
/// ---
/// 
/// Resolves all information about a system
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "System",
    params(
        ("name" = String, Query),
    ),
    responses(
        (
            body = Vec<System>,
            description = "Returns a list of all systems",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Query(filter):  Query<SystemSearchQuery>,
) -> Result<impl IntoResponse> {
    let entry = list(
        &state.postgres,
        filter,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
