use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_client::EveApiClient;

use crate::api_docs::{InternalServerError, NotFound};
use crate::proxy::error::Result;
use crate::state::AppState;

/// Proxy List
/// 
/// - Alternative route: `/latest/proxy/list`
/// - Alternative route: `/v1/proxy/list`
/// 
/// ---
/// 
/// Proxies requests to the EVE-API.
///
/// Returns an array of values.
/// 
/// This route does NOT support authenticated routes.
/// Use `/proxy/list/auth/{character/CharacterId}|{corporation/CorporationId}` for authenticated requests.
/// 
#[utoipa::path(
    get,
    path = "/list/{*path}",
    tag = "Proxy",
    params(
        ("*path" = String, Path, description = "Path to call on the EVE-API"),
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
) -> Result<impl IntoResponse> {
    let api_client = EveApiClient::new(state.eve_api_metric)?;
    let data = api_client
        .fetch_page::<serde_json::Value>(eve_path)
        .await?;

    Ok(
        (
            StatusCode::OK,
            Json(data),
        )
        .into_response()
    )
}
