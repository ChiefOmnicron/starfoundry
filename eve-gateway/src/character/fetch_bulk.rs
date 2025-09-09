mod service;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::character::CharacterInfo;
use crate::state::AppState;

use crate::character::error::Result;

pub use self::service::*;

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
    let entry = fetch_bulk(&state.postgres, character_ids).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
