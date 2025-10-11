use axum::extract::{Path, Request, State};
use axum::response::{IntoResponse, Redirect};
use reqwest::header::{CONTENT_TYPE, LOCATION};
use reqwest::StatusCode;
use serde::Deserialize;

use crate::state::AppState;
use crate::error::Result;
use crate::client::mtls_client;

#[axum::debug_handler]
pub async fn catch_all(
    State(state): State<AppState>,
    Path(path):   Path<String>,
    request:      Request,
) -> Result<impl IntoResponse> {
    // special routes that do not need authentication, or do not follow the
    // path syntax
    match path.as_ref() {
        ".well-known/jwks" |
        "auth/token" => {
            if let Some(x) = state.routes.get("/auth") {
                let mut url = x.service_url.clone();
                url.set_path(request.uri().path());
                url.set_query(request.uri().query());

                let client = mtls_client()?;
                let response = client
                    .get(url)
                    .headers(request.headers().clone())
                    .send()
                    .await?;

                if response.status().is_success() {
                    let status = response.status();
                    let body = response.text().await?;

                    return Ok((
                        status,
                        [("Content-Type", "application/json")],
                        body,
                    ).into_response());
                } else {
                    return Ok((
                        response.status(),
                    ).into_response());
                }
            } else {
                // TODO: better error handling
                return Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                ).into_response());
            }
        },
        "auth/login" => {
            if let Some(x) = state.routes.get("/auth") {
                let mut url = x.service_url.clone();
                url.set_path(request.uri().path());
                url.set_query(request.uri().query());

                let client = mtls_client()?;
                let response = client
                    .get(url)
                    .headers(request.headers().clone())
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
            }
        },
        "auth/callback" => {
            if let Some(x) = state.routes.get("/auth") {
                let mut url = x.service_url.clone();
                url.set_path(request.uri().path());
                url.set_query(request.uri().query());

                let client = mtls_client()?;
                let response = client
                    .get(url)
                    .headers(request.headers().clone())
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
            }
        },
        _ => ()
    };

    return Ok((
        StatusCode::ALREADY_REPORTED,
    ).into_response())
}

#[derive(Debug, Deserialize)]
struct AuthLoginResponse {
    url: String,
}

#[derive(Debug, Deserialize)]
struct AuthCallbackResponse {
    url:           String,
    refresh_token: String,
}
