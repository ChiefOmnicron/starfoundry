use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::HOST;
use axum::response::{IntoResponse, Redirect};

use crate::api_docs::InternalServerError;
use crate::auth::error::{AuthError, Result};
use crate::eve_client::EveApiClient;
use crate::state::AppState;
use axum::Json;

/// Login Main
/// 
/// - Alternative route: `/latest/auth/login`
/// - Alternative route: `/v1/auth/login`
/// 
/// ---
/// 
/// Logs in a new main character.
/// For alt characters or corporations the endpoints `/login/character` or
/// `/login/corporation` should be used.
/// 
/// 
/// The `HOST` header from the requester MUST match a host in the config file,
/// otherwise an error will be returned.
/// The API will return a `BAD_REQUEST` in that case, without giving further
/// information.
/// The service logs will contain more information.
/// 
/// It will return a redirect to the EVE-SSO.
/// The requester should follow the redirect.
/// 
/// After a successful login, the user will be redirect to the `/callback` route
/// to complete the login.
/// 
#[utoipa::path(
    get,
    path = "/login",
    tag = "Auth",
    responses(
        (
            status = TEMPORARY_REDIRECT,
            description = "Redirects to the Eve Login Server",
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
            INSERT INTO login_attempt (domain, credential_type)
            VALUES ($1, 'CHARACTER')
            RETURNING token
        ",
            host,
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
        //StatusCode::TEMPORARY_REDIRECT,
        //Redirect::temporary(&url),
        StatusCode::OK,
        Json(serde_json::json!({
            "url": url,
        }))
    ).into_response())
}
