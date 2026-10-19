use std::collections::HashMap;

use axum::Json;
use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use reqwest::header::HOST;

use crate::auth::GatewayExtractIdentity;
use crate::catch_all::add_headers;
use crate::error::Result;
use crate::state::AppState;
use crate::{LoginRedirect, SERVICE_NAME, StarFoundryApiClient};

pub async fn catch_all_auth_login_corporation(
    identity: GatewayExtractIdentity,
    headers: HeaderMap,
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    let host = if let Some(x) = headers.get(HOST) {
        x
    } else {
        return Ok((StatusCode::BAD_REQUEST).into_response())
    };

    if let Some(x) = state
        .routes
        .get("auth")
    {
        let mut url = x
            .service_url
            .clone();
        url.set_path("/auth/login/corporation");

        let mut headers = HeaderMap::new();
        add_headers(
            &mut headers,
            host.clone(),
            identity
                .character_info
                .character_id,
            identity
                .character_info
                .corporation_id,
            identity
                .character_info
                .alliance_id,
            identity.is_admin,
        );

        let response = StarFoundryApiClient::new_raw(SERVICE_NAME)?
            .get(url)
            .headers(headers)
            .query(&query)
            .send()
            .await?;

        if response
            .status()
            .is_success()
        {
            let body: LoginRedirect = response
                .json()
                .await?;

            Ok((
                StatusCode::OK,
                Json(serde_json::json!({
                    "url": body.url,
                })),
            )
                .into_response())
        } else {
            Ok((response.status()).into_response())
        }
    } else {
        // TODO: better error handling
        Ok((StatusCode::BAD_GATEWAY).into_response())
    }
}
