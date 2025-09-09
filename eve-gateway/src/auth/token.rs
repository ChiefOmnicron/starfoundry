use axum_extra::extract::CookieJar;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use base64::prelude::*;
use reqwest::header::HOST;
use serde_json::json;
use sha2::{Digest, Sha256};
use utoipa::ToSchema;

use crate::api_docs::Unauthorized;
use crate::auth::{AccessTokenClaims, RefreshTokenClaims};
use crate::auth::error::{AuthError, Result};
use crate::character::CharacterInfo;
use crate::state::AppState;

/// Access Token
/// 
/// - Alternative route: `/latest/auth/token`
/// - Alternative route: `/v1/auth/token`
/// 
/// ---
/// 
/// Returns a new JWT-Token, if the given `refresh_token` is valid.
/// 
/// It is recommended to store the JWT-Token in memory.
/// 
/// The token must be added into the `AUTHORIZATION` Header before sending a
/// request.
/// 
#[utoipa::path(
    get,
    path = "/token",
    tag = "Auth",
    responses(
        (
            status = OK,
            description = "Returns the access token",
            content_type = "application/json",
            body = AccessToken,
        ),
        Unauthorized,
    ),
)]
pub async fn token(
    State(state): State<AppState>,
    header:       HeaderMap,
    jar:          CookieJar,
) -> Result<impl IntoResponse> {
    let host = if let Some(x) = header.get(HOST) {
        x.to_str().unwrap_or_default()
    } else {
        tracing::error!("{HOST} header not present");
        return Ok((
            StatusCode::BAD_REQUEST,
        ).into_response())
    };

    let domain_config = if let Some(x) = state.auth_domains.get(host) {
        x
    } else {
        tracing::error!("'{host}' is not in the list of valid domains");
        return Ok((
            StatusCode::BAD_REQUEST,
        ).into_response())
    };

    if let Some(x) = jar.get("refresh_token").map(|x| x.value()) {
        let token_data = RefreshTokenClaims::verify(x)?;

        let refresh_token_hash = Sha256::digest(x.as_bytes());
        let refresh_token_hash = BASE64_STANDARD.encode(&refresh_token_hash);

        let character = sqlx::query!("
                SELECT
                    c.character_id,
                    c.character_name,
                    c.corporation_id,
                    c.corporation_name,
                    c.alliance_id,
                    c.alliance_name
                FROM jwt_refresh_token jrt
                JOIN character c ON jrt.character_id = c.character_id
                WHERE token_hash = $1
                AND jrt.character_id = $2
            ",
                refresh_token_hash,
                *token_data.claims.sub,
            )
            .fetch_optional(&state.postgres)
            .await
            .map_err(AuthError::GetRefreshTokenError)?;

        if let Some(record) = character {
            let character_info = CharacterInfo {
                character_name:   record.character_name,
                character_id:     record.character_id.into(),

                corporation_name: record.corporation_name,
                corporation_id:   record.corporation_id.into(),

                alliance_name:    record.alliance_name,
                alliance_id:      record.alliance_id.map(Into::into),
            };
            let character_id = record.character_id;

            let access_token = AccessTokenClaims::new(
                character_id.into(),
                character_info,
                domain_config.admins.contains(&character_id.into()),
                host.into(),
            )?;

            Ok((
                StatusCode::OK,
                Json(json!({
                    "access_token": access_token
                }))
            ).into_response())
        } else {
            Ok((
                StatusCode::UNAUTHORIZED
            ).into_response())
        }
    } else {
        Ok((
            StatusCode::UNAUTHORIZED
        ).into_response())
    }
}

// only needed as a type hint for utoipa
#[allow(dead_code)]
#[derive(ToSchema)]
#[schema(
    example = json!({
        "access_token": ""
    })
)]
struct AccessToken {
    access_token: String,
}
