use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::header::HOST;
use starfoundry_lib_eve_client::EveApiClient;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::InternalServerError;
use crate::auth::error::{AuthError, Result};
use crate::state::AppState;

/// Login Alt
/// 
/// Alternative route: `/latest/auth/login/character`
/// Alternative route: `/v1/auth/login/character`
/// 
/// ---
/// 
/// Logs in an alt character.
/// For main characters or corporations the endpoints `/login` or `/login/corporation` should be used
/// 
#[utoipa::path(
    get,
    path = "/login/character",
    tag = "Auth",
    responses(
        (
            status = TEMPORARY_REDIRECT,
            description = "Redirects to the Eve Login Server",
            body = String,
            content_type = "text/plain",
            example = json!("https://login.eveonline.com/v2/oauth/authorize/")
        ),
        InternalServerError,
    ),
)]
pub async fn login_character(
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

    let domain = if let Some(x) = state.auth_domains.get(host) {
        x
    } else {
        tracing::error!("'{host}' is not in the list of valid domains");
        return Ok((
            StatusCode::BAD_REQUEST,
        ).into_response())
    };

    let token = sqlx::query!("
            INSERT INTO login_attempt (domain, character_id, credential_type)
            VALUES ($1, $2, 'ALT_CHARACTER')
            RETURNING token
        ",
            host,
            *identity.character_id,
        )
        .fetch_one(&state.postgres)
        .await
        .map_err(AuthError::InsertLoginAttempt)?
        .token;

    let url = EveApiClient::auth_uri(
            &token.to_string(),
            &domain.character_scopes.join(" "),
        )?
        .to_string();

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "url": url,
        }))
    ).into_response())
}
