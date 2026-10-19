use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Redirect};
use reqwest::StatusCode;

use crate::error::Result;
use crate::state::AppState;
use crate::{LoginRedirect, SERVICE_NAME, StarFoundryApiClient};

pub async fn catch_all_auth_login(
    headers: HeaderMap,
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    if let Some(x) = state
        .routes
        .get("auth")
    {
        let mut url = x
            .service_url
            .clone();
        url.set_path("/auth/login");

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

            Ok((StatusCode::TEMPORARY_REDIRECT, Redirect::temporary(&body.url))
                .into_response())
        } else {
            Ok((response.status(),).into_response())
        }
    } else {
        // TODO: better error handling
        Ok((StatusCode::BAD_GATEWAY,).into_response())
    }
}
