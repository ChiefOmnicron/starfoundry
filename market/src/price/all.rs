use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;
use utoipa::ToSchema;

use crate::AppState;
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::price::error::{PriceError, Result};

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
pub async fn api(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let data = all(&state.postgres).await?;

    if data.is_empty() {
        Ok((StatusCode::NO_CONTENT, Json(data)).into_response())
    } else {
        Ok((StatusCode::OK, Json(data)).into_response())
    }
}

pub async fn all(pool: &PgPool) -> Result<Vec<PriceResponse>> {
    let prices = sqlx::query!(
        "
            SELECT
                type_id,
                adjusted_price,
                average_price
            FROM market_price
        "
    )
    .fetch_all(pool)
    .await
    .map_err(PriceError::BulkMarketPrice)?
    .into_iter()
    .map(|x| {
        PriceResponse {
            adjusted_price: x.adjusted_price,
            average_price:  x.average_price,
            type_id:        x
                .type_id
                .into(),
        }
    })
    .collect::<Vec<_>>();

    Ok(prices)
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct PriceResponse {
    pub type_id:        TypeId,
    pub adjusted_price: f64,
    pub average_price:  f64,
}
