use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use reqwest::header::{CONTENT_TYPE, LOCATION};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

use crate::client::mtls_client;
use crate::state::AppState;
use crate::error::Result;

pub async fn catch_all_auth_callback(
    headers:      HeaderMap,
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    if let Some(x) = state.routes.get("auth") {
        let mut url = x.service_url.clone();
        url.set_path("/auth/callback");

        let client = mtls_client()?;
        let response = client
            .get(url)
            .headers(headers)
            .query(&query)
            .send()
            .await?;

        if response.status().is_success() {
            let body: AuthCallbackResponse = response.json().await?;

            return Ok((
                StatusCode::FOUND,
                [(
                    LOCATION,
                    (&format!("{}?refresh_token={}", body.url, body.refresh_token)),
                ), (
                    CONTENT_TYPE,
                    &"application/json".to_string(),
                )],
            ).into_response())
        } else {
            return Ok((
                response.status(),
            ).into_response());
        }
    } else {
        // TODO: better error handling
        return Ok((
            StatusCode::BAD_GATEWAY,
        ).into_response());
    }
}

#[derive(Debug, Deserialize)]
struct AuthCallbackResponse {
    url:           String,
    refresh_token: String,
}
