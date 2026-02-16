use serde::{Deserialize, Serialize};
use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{StructureId, TypeId};
use utoipa::{IntoParams, ToSchema};

use crate::Result;

pub trait MarketApiClientOrder: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn bulk_latest_orders(
        &self,
        request: MarketBulkRequest,
    ) -> Result<Vec<MarketBulkResponse>> {
        self
            .post(
                "markets/bulk",
                request,
            )
            .await
            .map_err(Into::into)
    }
}

/// Bulk request for resolving prices
/// 
/// Either `appraisal` or `item_list` must be set
/// 
#[derive(Debug, Default, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct MarketBulkRequest {
    pub strategy: BuyStrategy,
    pub markets:  Vec<StructureId>,

    pub appraisal: Option<String>,
    pub item_list: Option<Vec<MarketItemList>>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct MarketBulkResponse {
    pub source:             StructureId,
    pub type_id:            i32,
    /// number of units that should be bought
    pub quantity:           u64,
    /// price per units
    pub price:              f64,
    /// if set to true, then there is no market to fulfill the request
    pub insufficient_data:  bool,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, IntoParams)]
pub struct MarketItemList {
    pub type_id:  TypeId,
    pub quantity: i32,
}

/// Different strategies for buying materials
/// 
#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    Deserialize, Serialize, ToSchema,
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum BuyStrategy {
    /// Acts like the in-game multi buy window
    /// 
    /// Advantages:
    /// - faster
    /// 
    /// Disadvantages:
    /// - can only buy from one market
    /// - no support for hauling costs
    /// - if a market does not have enough of the requested item type, it will
    ///   use the last price value
    MultiBuy,
    /// Looks at multiple markets in a detailed view
    /// 
    /// Advantages:
    /// - can buy from multiple markets
    /// - considers hauling costs
    /// 
    /// Disadvantages:
    /// - slower
    /// - depending on how old the market data is, the results may no longer be
    ///   valid
    SmartBuy,
}

impl Default for BuyStrategy {
    fn default() -> Self {
        Self::MultiBuy
    }
}
