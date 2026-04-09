use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_market::MarketVirtualRequest;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::market::error::Result;
use crate::market::virtual_market;

/// Update Virtual
/// 
/// - Alternative route: `/latest/markets/virtual`
/// - Alternative route: `/v1/markets/virtual`
/// 
/// ---
/// 
/// Updates the virtual remaining quantity for market orders
/// 
#[utoipa::path(
    post,
    path = "/virtual",
    tag = "Markets",
    request_body = Vec<MarketVirtualRequest>,
    responses(
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
    Json(request): Json<Vec<MarketVirtualRequest>>,
) -> Result<impl IntoResponse> {
    virtual_market(
            &state.postgres,
            request,
        ).await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(()),
        )
        .into_response()
    )
}
