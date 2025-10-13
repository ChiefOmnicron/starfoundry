use axum::extract::{Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use std::collections::HashMap;

use crate::client::mtls_client;
use crate::state::AppState;
use crate::error::Result;

pub async fn catch_all_store_general_info(
    headers:      HeaderMap,
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
) -> Result<impl IntoResponse> {
    if let Some(x) = state.routes.get("store") {
        let mut url = x.service_url.clone();
        url.set_path("/general/info");

        let client = mtls_client()?;
        let response = client
            .get(url)
            .headers(headers)
            .query(&query)
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
            StatusCode::BAD_GATEWAY,
        ).into_response());
    }
}
