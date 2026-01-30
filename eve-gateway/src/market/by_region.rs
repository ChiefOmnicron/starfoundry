use axum::extract::Path;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::market::Market;
use starfoundry_lib_types::RegionId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::eve_client::EveApiClient;
use crate::market::error::Result;

/// Fetch Market for a region
/// 
/// - Alternative route: `/latest/market/region/{RegionId}`
/// - Alternative route: `/v1/market/region/{RegionId}`
/// 
/// ---
/// 
/// Resolves the market data for the given region
/// 
#[utoipa::path(
    get,
    path = "/region/{RegionId}",
    tag = "Market",
    params(
        RegionId,
    ),
    responses(
        (
            body = Vec<Market>,
            description = "Market data for the region",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    Path(region_id): Path<RegionId>,
) -> Result<impl IntoResponse> {
    let api_client = EveApiClient::new()?;

    let path = format!("latest/markets/{region_id}/orders");
    let market_data = api_client
        .fetch_page::<Market>(&path)
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
