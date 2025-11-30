use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize, Deserializer};
use starfoundry_libs_types::{StationId, OrderId, LocationId, TypeId, RegionId};

use crate::{Cache, Error, EveApiClient};

impl EveApiClient {
    /// Fetches market information for the given [RegionId].
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [EveClient] is not valid
    /// - If the character does not have access to the structure
    /// - If the structure does not exist
    /// - If the [RegionId] is not a valid id
    /// 
    /// # Returns
    /// Information about the structure
    /// 
    pub async fn market_by_region(
        &self,
        region_id: &RegionId,
    ) -> Result<Vec<Market>, Error> {
        tracing::info!("Start market_region for {}", region_id);
        let path = format!("latest/markets/{region_id}/orders");
        self
            .fetch_page::<Market>(&path, Cache::Ignore)
            .await
            .map_err(Into::into)
    }

    /// Fetches the market for the given [StationId].
    /// The station id must be larget than 1_000_000_000_000.
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [EveAuthClient] is not valid
    /// - If the character does not have access to the structure
    /// - If the structure does not exist
    /// - If the [StationId] is not a valid id
    /// 
    /// # Returns
    /// Information about the structure
    /// 
    pub async fn market_by_structure(
        &self,
        station_id: &StationId,
    ) -> Result<Vec<Market>, Error> {
        let path = format!("latest/markets/structures/{}", station_id);
        self
            .fetch_page_auth::<Market>(&path, Cache::Follow)
            .await
            .map_err(Into::into)
    }

    /// Fetches the market prices for all items
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [EveClient] is not valid
    //
    pub async fn market_history(
        &self,
        region_id: &RegionId,
        type_id:   &TypeId,
    ) -> Result<Vec<MarketHistory>, Error> {
        let path = format!(
            "latest/markets/{}/history?type_id={}",
            region_id,
            type_id
        );
        self
            .fetch::<Vec<MarketHistory>>(&path, Cache::Follow)
            .await
            .map_err(Into::into)
    }

    /// Fetches the market prices for all items
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [EveClient] is not valid
    //
    pub async fn market_prices(
        &self,
    ) -> Result<Vec<MarketPrice>, Error> {
        let path = "latest/markets/prices".to_string();

        self
            .fetch::<Vec<MarketPrice>>(&path, Cache::Follow)
            .await
            .map_err(Into::into)
    }
}

/// Holds information about a market entry
#[derive(Debug, Deserialize, Serialize)]
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
    /// Relevant for buy orders, how much needs to be at least fullfilled
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
#[derive(Clone, Copy, Debug, Deserialize, Serialize, PartialEq, Eq)]
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

/// Represents a market price
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketPrice {
    /// Type ID of the item
    pub type_id:       TypeId,
    /// Adjusted price of the item
    pub adjusted_price: f64,
    /// Average price of the item
    #[serde(default)]
    pub average_price:  f64,
}

/// Represents a market price
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketHistory {
    /// Average price value
    pub average: f32,
    /// Highest price value
    pub highest: f32,
    /// Lowest price value
    pub lowest:  f32,
    /// Lowest price value
    pub date:    String,
}
