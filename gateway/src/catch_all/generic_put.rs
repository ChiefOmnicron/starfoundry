use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use std::collections::HashMap;

use crate::auth::ExtractIdentity;
use crate::catch_all::add_headers;
use crate::client::mtls_client;
use crate::error::Result;
use crate::state::AppState;

pub async fn catch_all_generic_put(
    identity:     Option<ExtractIdentity>,
    headers:      HeaderMap,
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    Path(path):   Path<String>,
    Json(body):   Json<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let path = {
        if path.starts_with("/") {
            path.replacen("/", "", 1)
        } else {
            path
        }
    };

    let (path_front, path_end) = if path.contains("/") {
        match path.split_once("/") {
            Some(x) => x,
            None => {
                tracing::error!("no initial path");
                return Ok((
                    StatusCode::BAD_GATEWAY,
                ).into_response())
            }
        }
    } else {
        (path.as_ref(), path.as_ref())
    };

    if let Some(x) = state.routes.get(path_front) {
        let mut headers = headers;
        if x.require_auth {
            if let Some(identity) = identity {
                add_headers(
                    &mut headers,
                    identity.character_info.character_id,
                    identity.character_info.corporation_id,
                    identity.character_info.alliance_id,
                    identity.is_admin,
                );
            } else {
                return Ok((
                    StatusCode::UNAUTHORIZED,
                ).into_response())
            }
        };

        let mut url = x.service_url.clone();
        if x.drop_prefix {
            url.set_path(path_end);
        } else {
            url.set_path(&path);
        }

        let client = mtls_client()?;
        let response = client
            .put(url)
            .headers(headers)
            .query(&query)
            .json(&body)
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
