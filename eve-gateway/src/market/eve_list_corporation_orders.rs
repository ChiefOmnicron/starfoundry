use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::eve_market::MarketOrder;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CorporationId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_corporation_auth;
use crate::market::experimental_insert_into_cache::write_to_cache;

const SCOPE: &str = "esi-markets.read_corporation_orders.v1";

/// Fetch Character Orders
/// 
/// - Alternative route: `/latest/eve/market/orders/corporations`
/// - Alternative route: `/v1/eve/market/orders/corporations`
/// 
/// ---
/// 
/// Loads all open orders from a corporation
/// 
#[utoipa::path(
    get,
    path = "/market/orders/corporations",
    tag = "Market",
    responses(
        (
            body = Vec<MarketOrder>,
            description = "List of orders from the corporation",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    identity:        ExtractIdentity,
    State(state):    State<AppState>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_corporation_auth(
            &state.postgres,
            state.metric,
            identity.host()?,
            identity.character_id,
            identity.corporation_id.unwrap_or(CorporationId(0)),
            vec![
                SCOPE.into(),
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
        "latest/corporations/{}/orders",
        // TODO: refactor
        *identity.corporation_id.unwrap_or(CorporationId(0)),
    );
    let market_data = api_client
        .fetch_page_auth::<MarketOrder>(&path)
        .await?;

    let time = std::time::Instant::now();
    if let Err(_) = write_to_cache(
        &state.postgres,
        serde_json::to_value(market_data.clone()).unwrap_or_default(),
        format!("ORDERS_CORPORATION_{}", identity.corporation_id.unwrap_or(CorporationId(0))),
    ).await {
        tracing::error!("Error writing into cache");
    }
    tracing::info!("cache time: {}", time.elapsed().as_millis());

    if market_data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(market_data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(market_data),
            )
            .into_response()
        )
    }
}
