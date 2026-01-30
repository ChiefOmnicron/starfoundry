use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::market::MarketPrice;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::eve_client::EveApiClient;

/// Fetch Market for a region
/// 
/// - Alternative route: `/latest/market/prices`
/// - Alternative route: `/v1/market/prices`
/// 
/// ---
/// 
/// Returns the cost of all items
/// 
#[utoipa::path(
    get,
    path = "/prices",
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
) -> Result<impl IntoResponse> {
    let api_client = EveApiClient::new()?;

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

