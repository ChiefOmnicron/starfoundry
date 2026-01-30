use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::internal::error::{InternalError, Result};

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
        ("host" = String, Query),
    ),
    responses(
        (
            body = Vec<CharacterId>,
            description = "List of character ids that have registered with the given host",
            status = OK,
        ),
        (
            body = Vec<CharacterId>,
            description = "No character ids are registered for the host",
            status = NO_CONTENT,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    _identity:    ExtractIdentity,
    State(state): State<AppState>,
    Query(query): Query<FetchCharacterIdsQuery>
) -> Result<impl IntoResponse> {
    let character_ids = sqlx::query!("
            SELECT character_id
            FROM eve_credential
            WHERE domain = $1
        ",
            query.host,
        )
        .fetch_all(&state.postgres)
        .await
        .map_err(InternalError::FetchCharacter)?
        .into_iter()
        .map(|x| x.character_id)
        .collect::<Vec<_>>();

    if character_ids.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(character_ids)
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(character_ids)
            )
            .into_response()
        )
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FetchCharacterIdsQuery {
    host: String,
}
