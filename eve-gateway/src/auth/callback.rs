use axum::extract::{Query, State};
use axum::http::header::LOCATION;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use base64::prelude::*;
use reqwest::header::CONTENT_TYPE;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

use crate::api_docs::{BadRequest, InternalServerError};
use crate::auth::RefreshTokenClaims;
use crate::auth::error::{AuthError, Result};
use crate::eve_client::{EveApiClient, EveJwtToken};
use crate::character::refresh_character_in_db;
use crate::state::AppState;

const QUERY_PARAM_CODE: &str  = "code";
const QUERY_PARAM_STATE: &str = "state";

/// Callback
/// 
/// - Alternative route: `/latest/auth/callback`
/// - Alternative route: `/v1/auth/callback`
/// 
/// ---
/// 
/// This route is called after the user logs in into Eve per SSO.
/// The `refresh_token` will be stored.
/// 
/// The user will be redirected to the callback defined in the config file.
/// In the response will be a cookie `refresh_token` which can be used to obtain
/// an access_token from the service.
/// See `/token` for more information.
/// 
#[utoipa::path(
    get,
    path = "/callback",
    tag = "Auth",
    responses(
        (
            status = TEMPORARY_REDIRECT,
            description = "Redirects back to the app",
            content_type = "text/plain",
        ),
        BadRequest,
        InternalServerError,
    ),
)]
// TODO: add whitelist check
pub async fn callback(
    State(state):        State<AppState>,
    Query(query_params): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    let code = query_params
        .get(QUERY_PARAM_CODE)
        .ok_or(AuthError::InvalidEveLoginResponse)?;
    let state_param = query_params
        .get(QUERY_PARAM_STATE)
        .ok_or(AuthError::InvalidEveLoginResponse)?;
    let state_param = Uuid::from_str(&state_param)
        .map_err(|_| AuthError::InvalidEveLoginResponse)?;

    // fetch the state_param, it should match the token we inserted earlier
    // if it's not their, it's either a wrong request or the login window expired
    let (domain, credential_type, main_character_id) = sqlx::query!("
            SELECT domain, credential_type, character_id
            FROM   login_attempt
            WHERE  token = $1
        ",
            state_param,
        )
        .fetch_one(&state.postgres)
        .await
        .map(|x| (x.domain, x.credential_type, x.character_id))
        .map_err(AuthError::GetAccessTokenError)?;

    let auth_domain = if let Some(x) = state.auth_domains.get(&domain) {
        x
    } else {
        // TODO: redirect to an error page
        return Ok((
            StatusCode::BAD_REQUEST
        ).into_response());
    };

    let token = EveApiClient::access_token(code).await?;

    let token_claims = if let Ok(x) = token.validate(
        EveApiClient::oauth_jwt_keys_url()?,
        EveApiClient::client_id()?,
    ).await {
        x
    } else {
        return Ok((
            StatusCode::BAD_REQUEST,
        ).into_response());
    };

    // TODO: is the character_id in there even if a corporation was logged in
    let character_id = EveJwtToken::extract_character_id(token_claims.claims)?;

    if credential_type == "CORPORATION" {
        let corporation_id = sqlx::query!("
                SELECT corporation_id
                FROM character
                WHERE character_id = $1
            ",
                main_character_id,
            )
            .fetch_one(&state.postgres)
            .await
            .map_err(AuthError::UpdateLogin)?
            .corporation_id;

        sqlx::query!("
                INSERT INTO eve_credential (
                    character_id,
                    refresh_token,
                    character_main
                ) VALUES ($1, $2, $3)
            ",
                corporation_id,
                &token.refresh_token,
                main_character_id,
            )
            .execute(&state.postgres)
            .await
            .map_err(AuthError::InsertEveCredential)?;
    } else {
        sqlx::query!("
            INSERT INTO eve_credential (
                character_id,
                refresh_token,
                character_main
            ) VALUES ($1, $2, $3)
            ON CONFLICT (character_id) DO UPDATE
            SET refresh_token = EXCLUDED.refresh_token
        ",
            *character_id,
            &token.refresh_token,
            main_character_id,
        )
        .execute(&state.postgres)
        .await
        .map_err(AuthError::InsertEveCredential)?;
    }

    refresh_character_in_db(&state.postgres, character_id).await?;

    let refresh_token = RefreshTokenClaims::new(
        character_id,
    )?;

    let refresh_token_hash = Sha256::digest(refresh_token.as_bytes());
    let refresh_token_hash = BASE64_STANDARD.encode(&refresh_token_hash);
    sqlx::query!("
            INSERT INTO jwt_refresh_token (character_id, refresh_token, token_hash)
            VALUES ($1, $2, $3)
        ",
            *character_id,
            refresh_token,
            refresh_token_hash,
        )
        .execute(&state.postgres)
        .await
        .map_err(AuthError::InsertRefreshToken)?;

    Ok((
        StatusCode::FOUND,
        [(
            LOCATION,
            (&format!("{}?refresh_token={}", auth_domain.redirect, refresh_token)),
        ), (
            CONTENT_TYPE,
            &"application/json".to_string(),
        )],
    ).into_response())
}
