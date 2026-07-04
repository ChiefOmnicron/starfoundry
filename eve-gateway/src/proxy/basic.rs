use axum::http::HeaderMap;
use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_client::EveApiClient;

use crate::api_docs::{InternalServerError, NotFound};
use crate::proxy::error::{ProxyError, Result};
use crate::state::AppState;

/// Proxy List
/// 
/// - Alternative route: `/latest/proxy`
/// - Alternative route: `/v1/proxy`
/// 
/// ---
/// 
/// Proxies requests to the EVE-API.
///
/// Returns an array of values.
/// 
/// This route does NOT support authenticated routes.
/// Use `/proxy/auth/{character/CharacterId}|{corporation/CorporationId}` for authenticated requests.
/// 
#[utoipa::path(
    get,
    path = "/{*path}",
    tag = "Proxy",
    params(
        ("*path" = String, Path, description = "Path to call on the EVE-API"),
        ("query" = serde_json::Value, Query),
    ),
    responses(
        (
            body = Vec<serde_json::Value>,
            description = "Response from the EVE-API",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Path(eve_path): Path<String>,
    Query(query):   Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let mut api_url = EveApiClient::api_url()?;
    api_url.set_path(&eve_path);

    let api_client = EveApiClient::new(state.eve_api_metric)?;
    let response = api_client
        .send(api_url.clone(), &query)
        .await?;

    if response.status() == StatusCode::NO_CONTENT {
        return Ok(
            (
                StatusCode::NO_CONTENT,
            ).into_response()
        )
    }

    let mut response_headers = HeaderMap::new();
    let headers = response.headers();
    if let Some(x) = headers.get("x-pages") {
        response_headers.insert("x-pages", x.clone());
    } else {
        response_headers.insert("x-pages", 1.into());
    };

    let data: serde_json::Value = match response.json().await {
        Err(e) => {
            tracing::error!("Error parsing json, {}", e);
            return Err(ProxyError::ReqwestError(e, api_url));
        },
        Ok(x) => x,
    };

    Ok(
        (
            StatusCode::OK,
            response_headers,
            Json(data),
        ).into_response()
    )
}
