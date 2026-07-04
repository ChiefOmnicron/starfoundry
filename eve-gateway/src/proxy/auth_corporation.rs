use axum::extract::{Path, Query, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::proxy::error::{ProxyError, Result};
use crate::state::AppState;
use crate::utils::api_client_auth;
use starfoundry_lib_eve_client::EveApiClient;
use axum::http::HeaderMap;

/// Proxy List Auth Character
/// 
/// - Alternative route: `/latest/proxy/auth/corporations/{*path}`
/// - Alternative route: `/v1/proxy/auth/corporations/{*path}`
/// 
/// ---
/// 
/// Proxies requests to the EVE-API.
/// Do NOT include `/corporations/{CorporationId}` in the query parameter.
/// 
/// Example: `/proxy/auth/corporations/industry/jobs` to call
/// `corporations/{YourCorporationId}/industry/jobs` on the EVE-API.
///
/// Returns an array of values.
/// 
/// This route is only for authenticated requests.
/// 
#[utoipa::path(
    get,
    path = "/list/auth/corporations/{*path}",
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
        BadRequest,
        NotFound,
        InternalServerError,
        Unauthorized,
    ),
)]
pub async fn api(
    identity:       ExtractIdentity,
    State(state):   State<AppState>,
    Path(eve_path): Path<String>,
    Query(query):   Query<serde_json::Value>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_auth(
            &state.postgres,
            state.eve_api_metric,
            identity.host()?,
            identity.character_id,
            vec![
                Scope::try_from(eve_path.as_ref()).map(|x| x.as_permission())?,
            ],
        )
        .await?;

    let api_client = if let Some(x) = api_client {
        x
    } else {
        return Ok(
            (
                StatusCode::UNAUTHORIZED,
            )
            .into_response()
        )
    };

    let path = format!(
        "latest/corporations/{}/{}",
        identity.corporation_id,
        eve_path,
    );
    let mut api_url = EveApiClient::api_url()?;
    api_url.set_path(&path);

    let response = api_client
        .send_auth(api_url.clone(), &query)
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

enum Scope {
    Assets,
    Blueprints,
    IndustryJob,
    Orders,
}

impl Scope {
    pub fn as_permission(&self) -> String {
        match self {
            Self::Assets        => "esi-corporations.read_corporation_assets.v1",
            Self::Blueprints    => "esi-corporations.read_blueprints.v1",
            Self::IndustryJob   => "esi-industry.read_corporation_jobs.v1",
            Self::Orders        => "esi-markets.read_corporation_orders.v1",
        }.into()
    }
}

impl TryFrom<&str> for Scope {
    type Error = ProxyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "assets"        => Ok(Self::Assets),
            "blueprints"    => Ok(Self::Blueprints),
            "industry/jobs" => Ok(Self::IndustryJob),
            "orders"        => Ok(Self::Orders),
            _               => {
                tracing::error!("No scope for {}", value);
                Err(ProxyError::NoScopeFound)
            },
        }
    }
}
