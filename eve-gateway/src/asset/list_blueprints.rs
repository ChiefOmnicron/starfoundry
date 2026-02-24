use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::Blueprint;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::Result;
use crate::state::AppState;
use crate::asset::service::list_blueprints;

/// Fetch Character
/// 
/// - Alternative route: `/latest/assets/blueprints`
/// - Alternative route: `/v1/assets/blueprints`
/// 
/// ---
/// 
/// Fetches information about a character
/// 
#[utoipa::path(
    get,
    path = "/blueprint",
    tag = "Asset",
    params(
        CharacterId,
    ),
    responses(
        (
            body = Vec<Blueprint>,
            description = "List of all blueprints the character has access to",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):       State<AppState>,
    Path(character_id): Path<CharacterId>,
) -> Result<impl IntoResponse> {
    let entry = list_blueprints(&state.postgres, character_id).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
