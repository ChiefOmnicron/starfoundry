use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::proxy::error::{ProxyError, Result};
use crate::state::AppState;
use crate::utils::api_client_auth;

/// Proxy List Auth Character
/// 
/// - Alternative route: `/latest/proxy/list/auth/{*path}`
/// - Alternative route: `/v1/proxy/list/auth/{*path}`
/// 
/// ---
/// 
/// Proxies requests to the EVE-API.
/// 
/// Example: `/proxy/list/auth/markets/structures/{StructureId}` to call
/// `{YourCorporationId}/industry/jobs` on the EVE-API.
///
/// Returns an array of values.
/// 
/// This route is only for authenticated requests.
/// 
#[utoipa::path(
    get,
    path = "/list/auth/{*path}",
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
        "latest/{}",
        eve_path,
    );
    let data = api_client
        .fetch_page_auth::<serde_json::Value>(&path)
        .await?;

    Ok(
        (
            StatusCode::OK,
            Json(data),
        )
        .into_response()
    )
}

enum Scope {
    MarketStructure,
}

impl Scope {
    pub fn as_permission(&self) -> String {
        match self {
            Self::MarketStructure   => "esi-markets.structure_markets.v1",
        }.into()
    }
}

impl TryFrom<&str> for Scope {
    type Error = ProxyError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.starts_with("markets/structures/") {
            Ok(Self::MarketStructure)
        } else {
            Err(ProxyError::NoScopeFound)
        }
    }
}
