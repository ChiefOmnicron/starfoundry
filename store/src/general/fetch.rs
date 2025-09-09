use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::InternalServerError;
use crate::AppState;
use crate::config::{ProductUuid, ShopConfigInfo};

/// Fetch General information
/// 
/// - Alternative route: `/latest/general/info`
/// - Alternative route: `/v1/general/info`
/// 
/// ---
/// 
/// Fetches all general information about the store
/// 
#[utoipa::path(
    get,
    path = "/info",
    tag = "general",
    params(
        ProductUuid,
    ),
    responses(
        (
            body = ShopConfigInfo,
            description = "General store information",
            status = OK,
        ),
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let info = state.shop_config.info.clone();

    (
        StatusCode::OK,
        Json(info),
    )
}
