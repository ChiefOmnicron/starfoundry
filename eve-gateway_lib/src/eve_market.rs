mod market;
mod prices;

pub use self::market::*;
pub use self::prices::*;

use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::HOST;
use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID, HEADER_CORPORATION_ID};
use starfoundry_lib_types::{CharacterId, CorporationId, RegionId, StructureId};

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
        source:       String,
        character_id: CharacterId,
        structure_id: StructureId,
    ) -> Result<Vec<Market>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .fetch_auth(
                &format!("eve/market/player/{}", *structure_id),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_character_orders(
        &self,
        source:       String,
        character_id: CharacterId,
    ) -> Result<Vec<Market>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        self
            .fetch_auth(
                &format!("eve/market/orders/characters"),
                &(),
                headers,
            )
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_corporation_orders(
        &self,
        source:         String,
        character_id:   CharacterId,
        corporation_id: CorporationId,
    ) -> Result<Vec<Market>> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());
        headers.insert(HEADER_CORPORATION_ID, (*corporation_id).into());

        self
            .fetch_auth(
                &format!("eve/market/orders/corporations"),
                &(),
                headers,
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
