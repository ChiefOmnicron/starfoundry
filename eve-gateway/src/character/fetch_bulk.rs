use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::CharacterInfo;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::character::error::Result;
use crate::state::AppState;
use crate::character::service::fetch_character_bulk;

/// Fetch multiple characters
/// 
/// - Alternative route: `/latest/characters/bulk`
/// - Alternative route: `/v1/characters/bulk`
/// 
/// ---
/// 
/// Fetches information about the list of given character ids.
/// The response order is not guaranteed to be be the same order the ids came in
/// 
#[utoipa::path(
    post,
    path = "/bulk",
    tag = "Character",
    request_body = Vec<CharacterId>,
    responses(
        (
            body = Vec<CharacterInfo>,
            description = "General information about the character",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):        State<AppState>,
    Json(character_ids): Json<Vec<CharacterId>>,
) -> Result<impl IntoResponse> {
    let entry = fetch_character_bulk(&state.postgres, character_ids).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
