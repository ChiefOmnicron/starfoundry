use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_types::{StructureId, TypeId};
use utoipa::{IntoParams, ToSchema};

use crate::{GasDecompressionEfficiency, OreReprocessingEfficiency};

/// Bulk request for resolving prices
/// 
/// Either `item_list_str` or `item_list` must be set
/// 
#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct MarketBulkRequest {
    pub strategy:           MarketStrategy,
    pub markets:            Vec<StructureId>,
    #[serde(default)]
    pub virtual_market:     bool,

    pub item_list:          Option<Vec<MarketItem>>,
    pub item_list_str:      Option<String>,
    pub smart_buy_config:   Option<SmartBuyConfig>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketBulkResponse {
    pub source:             StructureId,
    /// item information
    pub item:               Item,
    /// number of units that should be bought
    pub quantity:           u64,
    /// selected price
    pub price:              f64,
    /// if set to true, then there is no market to fulfill the request
    pub insufficient_data:  bool,
    /// time when the market was last fetched
    pub last_fetch:         Option<NaiveDateTime>,
    /// additional price information
    pub buy_price:          Option<MarketPrice>,
    pub sell_price:         Option<MarketPrice>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketPrice {
    pub max:            f64,
    pub min:            f64,
    pub total_orders:   i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct MarketItem {
    pub type_id:    TypeId,
    pub quantity:   i32,
}

/// Different strategies for buying materials
/// 
#[derive(
    Clone, Copy, Debug, Default, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    Deserialize, Serialize, ToSchema,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarketStrategy {
    /// Creates an appraisal for the given items
    /// 
    Appraisal,
    /// Acts like the in-game multi buy window
    /// 
    #[default]
    MultiBuy,
    /// Looks at multiple markets in a detailed view
    /// 
    SmartBuy,
}

#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct SmartBuyConfig {
    // gas decompression is active
    pub gas_decompression:      Option<GasDecompressionEfficiency>,
    // mineral compression is active
    pub mineral_compression:    Option<OreReprocessingEfficiency>,
}
