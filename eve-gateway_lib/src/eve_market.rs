mod market;
mod order;
mod prices;

pub use self::market::*;
pub use self::order::*;
pub use self::prices::*;

use starfoundry_lib_gateway::ApiClient;
use starfoundry_lib_types::{RegionId, StructureId};

use crate::error::Result;

pub trait EveGatewayApiClientEveMarket: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_market_by_region(
        &self,
        region_id: RegionId,
    ) -> Result<Vec<Market>> {
        self
            .fetch(&format!("eve/market/region/{}", *region_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_market_by_player(
        &self,
        structure_id: StructureId,
    ) -> Result<Vec<Market>> {
        self
            .fetch_auth(
                &format!("eve/market/player/{}", *structure_id),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_character_orders(
        &self,
    ) -> Result<Vec<MarketOrder>> {
        self
            .fetch_auth(
                &format!("eve/market/orders/characters"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_corporation_orders(
        &self,
    ) -> Result<Vec<MarketOrder>> {
        self
            .fetch_auth(
                &format!("eve/market/orders/corporations"),
                &(),
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_prices(
        &self,
    ) -> Result<Vec<MarketPrice>> {
        self
            .fetch(
                &format!("eve/market/prices"),
                &(),
            )
            .await
            .map_err(Into::into)
    }
}
