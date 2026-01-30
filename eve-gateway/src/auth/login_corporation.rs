use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use reqwest::header::HOST;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::InternalServerError;
use crate::auth::error::{AuthError, Result};
use crate::eve_client::EveApiClient;
use crate::state::AppState;

/// Login Corporation
/// 
/// Alternative route: `/latest/auth/login/corporation`
/// Alternative route: `/v1/auth/login/corporation`
/// 
/// ---
/// 
/// Logs in a corporation.
/// For main characters or alt characters the endpoints `/login` or `/login/character` should be used
/// 
#[utoipa::path(
    get,
    path = "/login/corporation",
    tag = "Auth",
    responses(
        (
            status = OK,
            description = "Redirects to the Eve Login Server",
            body = String,
            content_type = "text/plain",
            example = json!("https://login.eveonline.com/v2/oauth/authorize/")
        ),
        InternalServerError,
    ),
)]
pub async fn login_corporation(
    identity:     ExtractIdentity,
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
            INSERT INTO login_attempt (domain, character_id, credential_type)
            VALUES ($1, $2, 'CORPORATION')
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
            &domain.corporation_scopes.join(" "),
        )?
        .to_string();

    Ok((
        StatusCode::OK,
        Json(serde_json::json!({
            "url": url,
        }))
    ).into_response())
}
