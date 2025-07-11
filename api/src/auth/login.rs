use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use starfoundry_libs_eve_api::EveApiClient;

use crate::api_docs::InternalServerError;
use crate::AppState;
use crate::auth::error::{AuthError, Result};

/// Login Main
/// 
/// Alternative route: `/latest/auth/login`
/// Alternative route: `/v1/auth/login`
/// 
/// ---
/// 
/// Logs in a new main character.
/// For alt characters or corporations the endpoints `/login/alt` or `/login/corporation` should be used
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
) -> Result<impl IntoResponse> {
    let pool = state.pool.clone();

    let token = sqlx::query!("
            INSERT INTO credential (credential_type)
            VALUES ('CHARACTER')
            RETURNING token
        ")
        .fetch_one(&pool)
        .await
        .map_err(AuthError::InsertTokenError)?
        .token;

    let url = EveApiClient::auth_uri(
            &token.to_string(),
            &crate::auth::ESI_CHARACTER.join(" ")
        )
        .map_err(AuthError::EveApiError)?
        .to_string();

    Ok((
        StatusCode::TEMPORARY_REDIRECT,
        Redirect::temporary(&url),
    ))
}
