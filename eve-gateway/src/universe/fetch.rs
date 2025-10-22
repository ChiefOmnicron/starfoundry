mod model;
mod service;

pub use self::model::System;
pub use self::service::fetch;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_types::{SystemId, TypeId};

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::universe::error::Result;

/// Fetch System
/// 
/// - Alternative route: `/latest/universe/systems/{SystemId}`
/// - Alternative route: `/v1/universe/systems/{SystemId}`
/// 
/// ---
/// 
/// Resolves all information about a system
/// 
#[utoipa::path(
    get,
    path = "/systems/{SystemId}",
    tag = "Universe",
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
    let entry = fetch(
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
