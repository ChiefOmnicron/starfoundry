use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::eve_market::Market;
use starfoundry_lib_types::RegionId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::eve_client::EveApiClient;
use crate::market::error::Result;
use crate::state::AppState;
use crate::market::experimental_insert_into_cache::write_to_cache;

/// Fetch Market for a region
/// 
/// - Alternative route: `/latest/eve/market/region/{RegionId}`
/// - Alternative route: `/v1/eve/market/region/{RegionId}`
/// 
/// ---
/// 
/// Resolves the market data for the given region
/// 
#[utoipa::path(
    get,
    path = "/eve/region/{RegionId}",
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
    State(state):    State<AppState>,
    Path(region_id): Path<RegionId>,
) -> Result<impl IntoResponse> {
    let api_client = EveApiClient::new(state.metric)?;

    let path = format!("latest/markets/{region_id}/orders");
    let market_data = api_client
        .fetch_page::<Market>(&path)
        .await?;

    let time = std::time::Instant::now();
    if let Err(_) = write_to_cache(
        &state.postgres,
        serde_json::to_value(market_data.clone()).unwrap_or_default(),
        format!("STRUCTURE_REGION_{}", region_id),
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
