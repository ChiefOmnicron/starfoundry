use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::eve_market::MarketPrice;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::eve_client::EveApiClient;
use crate::state::AppState;

/// Fetch Market for a region
/// 
/// - Alternative route: `/latest/eve/market/prices`
/// - Alternative route: `/v1/eve/market/prices`
/// 
/// ---
/// 
/// Returns the cost of all items
/// 
#[utoipa::path(
    get,
    path = "/market/prices",
    tag = "Market",
    responses(
        (
            body = Vec<MarketPrice>,
            description = "Market prices for all items",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let api_client = EveApiClient::new(state.metric)?;

    let path = format!("latest/markets/prices");
    let market_data = api_client
        .fetch::<_, Vec<MarketPrice>>(&path, &())
        .await?;

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

