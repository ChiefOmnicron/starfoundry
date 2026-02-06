use std::fmt::Debug;

use axum::http::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_lib_gateway::{ApiClient, StarFoundryApiClient, Result as GatewayResult};
use url::Url;

use crate::error::{Error, Result};
use crate::MarketApiClientPrice;

pub const EVE_MARKET_API: &str = "STARFOUNDRY_MARKET_API_URL";

pub struct MarketClient(StarFoundryApiClient);

impl MarketClient {
    /// Creates a new [MarketClient] pulling the address form the ENV
    /// ENV: `STARFOUNDRY_MARKET_API_URL`
    /// 
    pub fn new(
        service: String,
    ) -> Result<Self> {
        let env = if let Ok(x) = std::env::var(EVE_MARKET_API) {
            x
        } else {
            return Err(Error::EnvNotSet(EVE_MARKET_API).into());
        };

        let api_url = Url::parse(&env).map_err(Error::UrlParseError)?;

        let api_client = StarFoundryApiClient::new(api_url, service)?;
        Ok(Self(api_client))
    }

    /// Creates a new [MarketClient] using the given address
    /// 
    pub fn new_with_address(
        service: String,
        address: String,
    ) -> Result<Self> {
        let api_url = Url::parse(&address).map_err(Error::UrlParseError)?;

        let api_client = StarFoundryApiClient::new(api_url, service)?;
        Ok(Self(api_client))
    }
}

impl ApiClient for MarketClient {
    async fn fetch<Q: Serialize, T>(
        &self,
        path:  impl Into<String>,
        query: &Q,
    ) -> GatewayResult<T>
    where
        T: DeserializeOwned {

        self.0
            .fetch(path, query)
            .await
            .map_err(Into::into)
    }

    async fn fetch_auth<Q: Serialize, T>(
        &self,
        path:       impl Into<String>,
        query:      &Q,
        header_map: HeaderMap,
    ) -> GatewayResult<T>
    where
        T: DeserializeOwned {

        self.0
            .fetch_auth(path, query, header_map)
            .await
            .map_err(Into::into)
    }

    async fn post<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> GatewayResult<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: DeserializeOwned {

        self.0
            .post(path, data)
            .await
            .map_err(Into::into)
    }
}

impl MarketApiClient for MarketClient {}

impl MarketApiClientPrice for MarketClient {}

/// Trait that should be implemented on all clients
/// The default implementation will be sufficient in most cases, overwriting
/// them is only recommended for mocking tests
pub trait MarketApiClient:
    ApiClient +
    MarketApiClientPrice {}
