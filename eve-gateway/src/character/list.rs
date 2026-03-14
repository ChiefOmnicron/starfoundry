use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::header::HOST;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::InternalServerError;
use crate::auth::error::Result;
use crate::state::AppState;
use crate::character::service::list_authed_characters;
use starfoundry_lib_eve_gateway::AuthedCharacterInfo;

/// Login Corporation
/// 
/// Alternative route: `/latest/characters/corporation`
/// Alternative route: `/v1/characters/corporation`
/// 
/// ---
/// 
/// Logs in a corporation.
/// For main characters or alt characters the endpoints `/login` or `/login/character` should be used
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "Character",
    responses(
        (
            status = OK,
            description = "List of characters that are authed",
            body = Vec<AuthedCharacterInfo>,
        ),
        InternalServerError,
    ),
)]
pub async fn api(
    identity:     ExtractIdentity,
    State(state): State<AppState>,
    header:       HeaderMap,
) -> Result<impl IntoResponse> {
    let host = if let Some(x) = header.get(HOST) {
        x.to_str().unwrap_or_default()
    } else {
        tracing::error!("{HOST} header not present");
        return Ok((
            StatusCode::BAD_REQUEST,
        ).into_response())
    };

    let entry = list_authed_characters(
            &state.postgres,
            state.eve_api_metric,
            identity.character_id,
            host.into(),
        )
        .await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
