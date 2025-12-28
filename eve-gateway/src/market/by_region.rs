use axum::Json;
use axum::extract::Path;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::{BuyOrder, Market};
use starfoundry_lib_types::{LocationId, OrderId, RegionId, TypeId};

use crate::api_docs::{InternalServerError, NotFound};
use crate::eve_client::EveApiClient;
use crate::market::error::Result;
use serde::{Deserialize, Deserializer};
use chrono::NaiveDateTime;

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
        .fetch_page::<MarketApi>(&path)
        .await?
        .into_iter()
        .map(|x| Market {
            duration:      x.duration,
            is_buy_order:  x.is_buy_order,
            issued:        x.issued,
            location_id:   x.location_id,
            min_volume:    x.min_volume,
            order_id:      x.order_id,
            price:         x.price,
            type_id:       x.type_id,
            volume_remain: x.volume_remain,
            volume_total:  x.volume_total,
        })
        .collect::<Vec<_>>();

    if market_data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
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

/// The EVE-API returns some unfavorable data types, always using them will cause
/// more issues, so this type is a wrapper type to properly parse the EVE-API result
/// 
#[derive(Debug, Deserialize)]
struct MarketApi {
    /// Duration of the market entry until it expires
    pub duration:      u32,
    /// Determines if the order is a buy or sell order
    #[serde(deserialize_with = "BuyOrder::from_bool")]
    pub is_buy_order:  BuyOrder,
    /// Date when the market entry was issued
    #[serde(deserialize_with = "from_datetime")]
    pub issued:        NaiveDateTime,
    /// Station the market entry was issued from
    pub location_id:   LocationId,
    /// Relevant for buy orders, how much needs to be at least fulfilled
    pub min_volume:    u32,
    /// Order id of the market entry
    pub order_id:      OrderId,
    /// Price the issuer wants
    pub price:         f32,
    /// TypeId of the item
    pub type_id:       TypeId,
    /// Volume that is still up
    pub volume_remain: u32,
    /// Total volume of the market order
    pub volume_total:  u32,
}

/// The API gives us a string for the date, we parse it into a NativeDateTime
pub fn from_datetime<'de, D>(
    deserializer: D
) -> Result<NaiveDateTime, D::Error>
    where
        D: Deserializer<'de> {

    let datetime: String = Deserialize::deserialize(deserializer)?;
    let datetime = NaiveDateTime::parse_from_str(
        &datetime, "%Y-%m-%dT%H:%M:%S%Z"
    )
    .unwrap_or_default();

    Ok(datetime)
}
