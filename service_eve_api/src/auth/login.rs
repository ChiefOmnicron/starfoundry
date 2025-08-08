use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::HOST;
use axum::response::{IntoResponse, Redirect};

use crate::api_docs::InternalServerError;
use crate::AppState;
use crate::auth::error::{AuthError, Result};
use crate::client::EveApiClient;
use uuid::Uuid;

const FIFTEEN_MINUTES_IN_SECS: u64 = 15 * 60;

/// Login Main
/// 
/// Alternative route: `/latest/auth/login`
/// Alternative route: `/v1/auth/login`
/// 
/// ---
/// 
/// Logs in a new main character.
/// For alt characters or corporations the endpoints `/login/character` or `/login/corporation` should be used
/// 
/// Upon a successful authentication, it will return a JWT-Token and a Refresh-Token.
/// The JWT-Token shall not be saved locally, and should stay in memory.
/// The Refresh-Token is returned as a cookie, and must be send to obtain a new JWT-Token.
/// 
#[utoipa::path(
    get,
    path = "/login",
    tag = "auth",
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
pub async fn login(
    State(state): State<AppState>,
    header: HeaderMap,
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
            INSERT INTO login_attempt (credential_type)
            VALUES ('CHARACTER')
            RETURNING token
        ")
        .fetch_one(&state.postgres)
        .await
        .map_err(AuthError::InsertTokenError)?
        .token;

    let url = EveApiClient::auth_uri(
            state.eve_api.client_id.clone(),
            state.eve_api.callback.clone(),
            &token.to_string(),
            &domain.character_scopes.join(" "),
        )
        .map_err(AuthError::EveApiError)?
        .to_string();

    Ok((
        StatusCode::TEMPORARY_REDIRECT,
        Redirect::temporary(&url),
    ).into_response())
}
