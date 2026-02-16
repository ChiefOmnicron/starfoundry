use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_market::{MarketBulkRequest, MarketBulkResponse};

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::market::error::Result;
use crate::market::service::bulk;

/// Bulk Market Data
/// 
/// - Alternative route: `/latest/markets/bulk`
/// - Alternative route: `/v1/markets/bulk`
/// 
/// ---
/// 
/// Bulk get data from the markets
/// 
/// The API has two strategies for finding prices.
/// - `MULTI_BUY` uses the same method as the in-game multibuy
///   window. If no market has enough units to fulfil the wanted materials
///   the highest price found will be used and the flag `insufficient_data` will
///   be set to `true`
/// - `SMART_BUY` factors in hauling cost and the prices of all available markets.
///   The feature is currently locked behind a feature flag
/// 
#[utoipa::path(
    post,
    path = "/bulk",
    tag = "Markets",
    request_body = MarketBulkRequest,
    responses(
        (
            body = Vec<MarketBulkResponse>,
            description = "List of all matching market entries",
            status = OK,
        ),
        (
            description = "No markets have the requested amount available",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
#[axum::debug_handler]
pub async fn api(
    State(state):  State<AppState>,
    Json(request): Json<MarketBulkRequest>,
) -> Result<impl IntoResponse> {
    let data = bulk(
            &state.pool,
            request,
        ).await?;

    if data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(data),
            )
            .into_response()
        )
    }
}
