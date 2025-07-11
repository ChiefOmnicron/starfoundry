use axum::extract::{Query, State};
use axum::http::header::{LOCATION, SET_COOKIE};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect};
use starfoundry_libs_eve_api::EveApiClient;
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

use crate::api_docs::{BadRequest, InternalServerError};
use crate::AppStateExtractor;
use crate::auth::error::AuthError;

/// Callback
/// 
/// This route is called after the user logs in into Eve per SSO.
/// It will save the information given.
/// After that it will redirect the user to the landing page of the webapp
/// and include a cookie `refresh_token` in it.
/// Additionally a JWT-Token is included, this shall not be stored in local
/// storage or anywhere else, it shall stay in memory
/// 
#[utoipa::path(
    get,
    path = "/callback",
    tag = "auth",
    responses(
        (
            status = TEMPORARY_REDIRECT,
            description = "Redirects to the Eve Login Server",
            body = String,
            content_type = "text/plain",
            example = json!("https://login.eveonline.com/v2/oauth/authorize/")
        ),
        BadRequest,
        InternalServerError,
    ),
)]
pub async fn callback(
    State(state): AppStateExtractor,
    Query(query_params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let pool = state.pool.clone();

    let code = query_params
        .get("code")
        .ok_or(AuthError::InvalidEveLoginResponse)?;
    let state_param = query_params
        .get("state")
        .ok_or(AuthError::InvalidEveLoginResponse)?;
    let state_param = Uuid::from_str(&state_param)
        .map_err(|_| AuthError::InvalidEveLoginResponse)?;

    let token = EveApiClient::access_token(code)
        .await
        .map_err(AuthError::EveApiError)?;

    if !token.validate() {
        return Err(AuthError::InvalidEveJwtToken);
    }

    let character_id = token
        .character_id()
        .map_err(AuthError::EveApiError)?;

    let login_attempt = sqlx::query!("
            SELECT token, credential_type
            FROM   credential
            WHERE  token = $1
        ",
            state_param,
        )
        .fetch_one(&pool)
        .await
        .map_err(AuthError::GetTokenError)?;

    // TODO: write the data away
    // TODO: return JWT token
    // TODO: return refresh token

    if login_attempt.credential_type == "CORPORATION" {
        let corporation_id = sqlx::query!("
                SELECT corporation_id
                FROM character
                WHERE character_id = $1
                ",
                    *character_id,
                )
                .fetch_one(&pool)
                .await
                .map_err(AuthError::UpdateLogin)?
                .corporation_id;

        sqlx::query!("
            UPDATE credential
                SET
                    character_id   = $1,
                    refresh_token  = $2,
                    access_token   = $3
                WHERE token = $4
            ",
                corporation_id,
                &token.refresh_token,
                &token.access_token,
                state_param
            )
            .execute(&pool)
            .await
            .map_err(AuthError::UpdateLogin)?;
    } else {
        sqlx::query!("
            UPDATE credential
            SET
                character_id  = $1,
                refresh_token = $2,
                access_token  = $3
            WHERE token = $4
        ",
            *character_id,
            &token.refresh_token,
            &token.access_token,
            state_param
        )
        .execute(&pool)
        .await
        .map_err(AuthError::UpdateLogin)?;
    }

    let temp_client = EveApiClient::new().unwrap();
    let character_info = temp_client
        .character_info_by_id(character_id)
        .await
        .unwrap();

    let eve_client = EveApiClient::new_with_refresh_token(
            character_id,
            character_info.corporation_id,
            token.refresh_token.clone(),
        )
        .map_err(AuthError::EveApiError)?;

    state.credential_cache
        .lock()
        .unwrap()
        .insert(character_id, eve_client);

    // TODO: validate the existance in the begining
    let redirect = std::env::var("REDIRECT").unwrap();

    if login_attempt.credential_type == "CORPORATION" ||
        login_attempt.credential_type == "CHARACTER_ALT" {

        let redirect = format!("{}/characters", redirect);

        return Ok((
            StatusCode::TEMPORARY_REDIRECT,
            Redirect::temporary(&redirect),
        ).into_response());
    }

    // TODO: Better method?
    // TODO: implement rolling refresh_tokens
    let refresh_token = Uuid::new_v4().to_string();

    sqlx::query!("
            INSERT INTO jwt_refresh_token (character_id, refresh_token)
            VALUES ($1, $2)
        ",
            *character_id,
            refresh_token,
        )
        .execute(&pool)
        .await
        .unwrap();

    Ok((
        StatusCode::TEMPORARY_REDIRECT,
        [(
            LOCATION,
            redirect,
        ), (
            SET_COOKIE,
            (&format!("refresh_token={}; HttpOnly; Secure; SameSite=None; Path=/; MaxAge=86400", refresh_token)).into(),
        )],
    ).into_response())
}
