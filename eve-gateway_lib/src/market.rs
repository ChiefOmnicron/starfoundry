mod market;
mod order;
mod prices;

pub use self::market::*;
pub use self::order::*;
pub use self::prices::*;

use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{RegionId, StructureId};

use crate::error::Result;

pub trait EveGatewayApiClientMarket: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn list_market_by_region(
        &self,
        region_id: RegionId,
    ) -> Result<Vec<Market>> {
        self
            .fetch(&format!("proxy/list/markets/{}/orders", *region_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_market_by_player(
        &self,
        structure_id: StructureId,
    ) -> Result<Vec<Market>> {
        self
            .fetch_auth(
                &format!("proxy/list/auth/markets/structures/{}", *structure_id),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn list_character_orders(
        &self,
    ) -> Result<Vec<MarketOrder>> {
        self
            .fetch_auth(
                "proxy/list/auth/characters/orders",
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
            .fetch_auth(
                "proxy/list/auth/corporations/orders",
                &(),
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
                "proxy/list/markets/prices",
                &(),
            )
            .await
            .map_err(Into::into)
    }
}
