use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Redirect};
use reqwest::StatusCode;
use serde::Deserialize;
use starfoundry_lib_gateway::StarFoundryApiClient;
use std::collections::HashMap;

use crate::error::Result;
use crate::SERVICE_NAME;
use crate::state::AppState;

pub async fn catch_all_auth_login(
    headers:      HeaderMap,
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    if let Some(x) = state.routes.get("auth") {
        let mut url = x.service_url.clone();
        url.set_path("/auth/login");

        let response = StarFoundryApiClient::new_raw(
                SERVICE_NAME,
            )?
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
