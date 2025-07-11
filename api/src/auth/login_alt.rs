use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use starfoundry_libs_eve_api::EveApiClient;

use crate::api_docs::InternalServerError;
use crate::AppStateExtractor;
use crate::auth::error::{AuthError, Result};

/// Login Alt
/// 
/// Logs in an alt character.
/// For main characters or corporations the endpoints `/login` or `/login/corporation` should be used
/// 
#[utoipa::path(
    get,
    path = "/login/alt",
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
pub async fn login_alt(
    State(state): AppStateExtractor,
) -> Result<impl IntoResponse> {
    let pool = state.pool.clone();

    let token = sqlx::query!("
            INSERT INTO credential (credential_type)
            VALUES ('CHARACTER_ALT')
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
