use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::response::IntoResponse;
use reqwest::header::HOST;
use reqwest::StatusCode;
use starfoundry_lib_gateway::StarFoundryApiClient;
use std::collections::HashMap;

use crate::auth::ExtractIdentity;
use crate::catch_all::add_headers;
use crate::error::Result;
use crate::state::AppState;
use crate::SERVICE_NAME;

#[axum::debug_handler]
pub async fn catch_all_generic_get(
    identity:     Option<ExtractIdentity>,
    header_map:   HeaderMap,
    State(state): State<AppState>,
    Query(query): Query<HashMap<String, String>>,
    Path(path):   Path<String>,
) -> Result<impl IntoResponse> {
    tracing::info!("[GET] - path: '{}'", path);
    let host = if let Some(x) = header_map.get(HOST) {
        x
    } else {
        return Ok(
            (
                StatusCode::BAD_REQUEST
            )
            .into_response()
        )
    };

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
        let mut headers = HeaderMap::new();
        if x.require_auth {
            if let Some(identity) = identity {
                add_headers(
                    &mut headers,
                    host.clone(),
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

        let response = StarFoundryApiClient::new_raw(
                SERVICE_NAME,
            )?
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
        tracing::error!("no target found for path '{}'", path);
        // TODO: better error handling
        return Ok((
            StatusCode::BAD_GATEWAY,
        ).into_response());
    }
}
