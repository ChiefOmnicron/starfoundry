use std::collections::HashMap;

use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use reqwest::header::{CONTENT_TYPE, LOCATION};
use serde::Deserialize;

use crate::error::Result;
use crate::state::AppState;
use crate::{SERVICE_NAME, StarFoundryApiClient};

pub async fn catch_all_auth_callback(
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
        url.set_path("/auth/callback");

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
            let body: AuthCallbackResponse = response
                .json()
                .await?;

            if let Some(x) = body.refresh_token {
                Ok((
                    StatusCode::FOUND,
                    [
                        (
                            LOCATION,
                            (&format!("{}?refresh_token={}", body.url, x)),
                        ),
                        (CONTENT_TYPE, &"application/json".to_string()),
                    ],
                )
                    .into_response())
            } else {
                Ok((
                    StatusCode::FOUND,
                    [
                        (
                            LOCATION,
                            (&format!(
                                "{}?additional_character=true",
                                body.url
                            )),
                        ),
                        (CONTENT_TYPE, &"application/json".to_string()),
                    ],
                )
                    .into_response())
            }
        } else {
            let body: AuthCallbackResponse = response
                .json()
                .await?;

            Ok((
                StatusCode::FOUND,
                [(LOCATION, format!("{}?success=false", body.url))],
            )
                .into_response())
        }
    } else {
        // TODO: better error handling
        Ok((StatusCode::BAD_GATEWAY).into_response())
    }
}

#[derive(Debug, Deserialize)]
struct AuthCallbackResponse {
    url:           String,
    refresh_token: Option<String>,
}
