use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_market::PriceResponse;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::price::error::Result;
use crate::price::service::all;

/// Bulk Price Data
/// 
/// - Alternative route: `/latest/prices`
/// - Alternative route: `/v1/prices`
/// 
/// ---
/// 
/// Bulk get data for prices
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "Prices",
    request_body = Vec<TypeId>,
    responses(
        (
            body = Vec<PriceResponse>,
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
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):  State<AppState>,
) -> Result<impl IntoResponse> {
    let data = all(
            &state.pool,
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
