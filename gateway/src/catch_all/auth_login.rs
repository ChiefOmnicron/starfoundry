use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Redirect};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

use crate::client::mtls_client;
use crate::state::AppState;
use crate::error::Result;

pub async fn catch_all_auth_login(
    headers:      HeaderMap,
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    if let Some(x) = state.routes.get("auth") {
        let mut url = x.service_url.clone();
        url.set_path("/auth/login");

        let client = mtls_client()?;
        let response = client
            .get(url)
            .headers(headers)
            .query(&query)
            .send()
            .await?;

        if response.status().is_success() {
            let body: AuthLoginResponse = response.json().await?;

            return Ok((
                StatusCode::TEMPORARY_REDIRECT,
                Redirect::temporary(&body.url),
            ).into_response());
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
struct AuthLoginResponse {
    url: String,
}
