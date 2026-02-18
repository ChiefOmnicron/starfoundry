use starfoundry_lib_types::{CharacterId, LocationId, OrderId, RegionId, TypeId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use chrono::NaiveDateTime;

use crate::utils::from_datetime;
use crate::eve_market::BuyOrder;

/// Holds information about a market entry
#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketOrder {
    /// Duration of the market entry until it expires
    pub duration:        u32,
    /// Determines if the order is a buy or sell order
    #[serde(deserialize_with = "BuyOrder::from_bool")]
    #[serde(serialize_with = "BuyOrder::to_bool")]
    #[serde(default)]
    pub is_buy_order:    BuyOrder,
    /// Date when the market entry was issued
    #[serde(deserialize_with = "from_datetime")]
    pub issued:          NaiveDateTime,
    /// Station the market entry was issued from
    pub location_id:     LocationId,
    /// Relevant for buy orders, how much needs to be at least fulfilled
    #[serde(default)]
    pub min_volume:      u32,
    /// Order id of the market entry
    pub order_id:        OrderId,
    /// Price the issuer wants
    pub price:           f32,
    /// Valid order range, numbers are ranges in jumps
    /// Allowed values: 1 10 2 20 3 30 4 40 5 region solarsystem station
    pub range:           String,
    /// Order id of the market entry
    pub region_id:       RegionId,
    /// TypeId of the item
    pub type_id:         TypeId,
    /// Volume that is still up
    pub volume_remain:   u32,
    /// Total volume of the market order
    pub volume_total:    u32,
    /// Total volume of the market order
    pub escrow:          Option<f32>,
    /// Total volume of the market order
    pub is_corporation:  Option<bool>,
    /// Total volume of the market order
    pub issued_by:       Option<CharacterId>,
    /// Total volume of the market order
    pub wallet_division: Option<i32>,
}
