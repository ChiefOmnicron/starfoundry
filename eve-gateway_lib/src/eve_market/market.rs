use chrono::NaiveDateTime;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use starfoundry_lib_types::{LocationId, OrderId, TypeId};
use utoipa::ToSchema;

use crate::utils::from_datetime;

/// Holds information about a market entry
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct Market {
    /// Duration of the market entry until it expires
    pub duration:       u32,
    /// Determines if the order is a buy or sell order
    #[serde(deserialize_with = "BuyOrder::from_bool")]
    #[serde(serialize_with = "BuyOrder::to_bool")]
    #[serde(default)]
    pub is_buy_order:   BuyOrder,
    /// Date when the market entry was issued
    #[serde(deserialize_with = "from_datetime")]
    pub issued:         NaiveDateTime,
    /// Station the market entry was issued from
    pub location_id:    LocationId,
    /// Relevant for buy orders, how much needs to be at least fulfilled
    #[serde(default)]
    pub min_volume:     u32,
    /// Order id of the market entry
    pub order_id:       OrderId,
    /// Price the issuer wants
    pub price:          f32,
    /// TypeId of the item
    pub type_id:        TypeId,
    /// Volume that is still up
    pub volume_remain:  u32,
    /// Total volume of the market order
    pub volume_total:   u32,
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
        deserializer: D,
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

    /// Convert it back into a bool when serialized
    pub fn to_bool<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
        where
            S: Serializer {

        match self {
            Self::Buy  => serializer.serialize_bool(true),
            Self::Sell => serializer.serialize_bool(false),
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

/// For corporate and character orders the field is only set to true and not to
/// false
impl Default for BuyOrder {
    fn default() -> Self {
        Self::Sell
    }
}
