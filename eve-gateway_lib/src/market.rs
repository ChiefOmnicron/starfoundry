use axum::http::{HeaderMap, HeaderValue};
use chrono::NaiveDateTime;
use reqwest::header::HOST;
use serde::{Deserialize, Deserializer, Serialize};
use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID};
use starfoundry_lib_types::{CharacterId, LocationId, OrderId, RegionId, StructureId, TypeId};
use utoipa::ToSchema;

use crate::error::Result;

/// Holds information about a market entry
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Market {
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

/// Determines if an market entry is a Buy or Sell order
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq, ToSchema)]
pub enum BuyOrder {
    /// Entry is a buy order
    Buy,
    /// Entry is a sell order
    Sell,
}

impl BuyOrder {
    /// The API gives us a bool if the entry is a buy order, so we parse
    /// it and press it into our enum
    pub fn from_bool<'de, D>(
        deserializer: D
    ) -> Result<Self, D::Error>
        where
            D: Deserializer<'de> {

        let val: bool = Deserialize::deserialize(deserializer)?;
        if val {
            Ok(Self::Buy)
        } else {
            Ok(Self::Sell)
        }
    }
}

impl From<BuyOrder> for bool {
    fn from(value: BuyOrder) -> Self {
        match value {
            BuyOrder::Buy  => true,
            BuyOrder::Sell => false,
        }
    }
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

pub trait EveGatewayApiClientMarket: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_market_by_region(
        &self,
        region_id: RegionId,
    ) -> Result<Vec<Market>> {
        self
            .fetch(&format!("market/region/{}", *region_id), &[])
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_market_by_player(
        &self,
        source:       String,
        character_id: CharacterId,
        structure_id: StructureId,
    ) -> Result<Vec<Market>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .fetch_auth(
                &format!("market/player/{}", *structure_id),
                &[],
                headers,
            )
            .await
            .map_err(Into::into)
    }
}
