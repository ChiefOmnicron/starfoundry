mod market;
mod order;
mod prices;

pub use self::market::*;
pub use self::order::*;
pub use self::prices::*;

use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{RegionId, StructureId};

use crate::error::Result;
use crate::ApiClientExtended;

pub trait EveGatewayApiClientMarket: ApiClient + ApiClientExtended {
    #[allow(async_fn_in_trait)]
    async fn list_market_by_region(
        &self,
        region_id: RegionId,
    ) -> Result<Vec<Market>> {
        self
            .fetch_page(&format!("proxy/markets/{}/orders", *region_id))
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_market_by_player(
        &self,
        structure_id: StructureId,
    ) -> Result<Vec<Market>> {
        self
            .fetch_page(
                &format!("proxy/auth/markets/structures/{}", *structure_id),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_character_orders(
        &self,
    ) -> Result<Vec<MarketOrder>> {
        self
            .fetch(
                "proxy/auth/characters/orders",
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_corporation_orders(
        &self,
    ) -> Result<Vec<MarketOrder>> {
        self
            .fetch_page(
                "proxy/auth/corporations/orders",
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_prices(
        &self,
    ) -> Result<Vec<MarketPrice>> {
        self
            .fetch(
                "proxy/markets/prices",
                &(),
            )
            .await
            .map_err(Into::into)
    }
}
