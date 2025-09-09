use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use starfoundry_lib_eve_api::EveApiClient;

use crate::api_docs::InternalServerError;
use crate::AppState;
use crate::auth::error::{AuthError, Result};

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
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let pool = state.pool.clone();

    let token = sqlx::query!("
            INSERT INTO credential (credential_type, character_id)
            VALUES ('CHARACTER_ALT', $1)
            RETURNING token
        ",
            0, // TODO: replace with whatever the character_id is
        )
        .fetch_one(&pool)
        .await
        .map_err(AuthError::InsertTokenError)?
        .token;

    let url = EveApiClient::auth_uri(
            &token.to_string(),
            &crate::auth::ESI_CHARACTER.join(" ")
        )?
        .to_string();

    Ok((
        StatusCode::TEMPORARY_REDIRECT,
        Redirect::temporary(&url),
    ))
}
