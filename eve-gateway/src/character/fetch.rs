mod service;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_types::CharacterId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;

use crate::character::error::Result;

pub use self::service::*;

/// Fetch Character
/// 
/// - Alternative route: `/latest/characters/{CharacterId}`
/// - Alternative route: `/v1/characters/{CharacterId}`
/// 
/// ---
/// 
/// Fetches information about a character
/// 
#[utoipa::path(
    get,
    path = "/{CharacterId}",
    tag = "Character",
    params(
        CharacterId,
    ),
    responses(
        (
            body = CharacterInfo,
            description = "General information about the character",
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
    let entry = fetch(&state.postgres, character_id).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
