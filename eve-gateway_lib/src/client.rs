use std::fmt::Debug;

use axum::http::HeaderMap;
use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_lib_gateway::{ApiClient, MtlsApiClient, Result as GatewayResult};
use url::Url;

use crate::error::{Error, Result};
use crate::{EveGatewayApiClient, EveGatewayApiClientAsset, EveGatewayApiClientIndustry, EveGatewayApiClientItem};
use crate::market::EveGatewayApiClientMarket;
use crate::contract::EveGatewayApiClientContract;

pub const ENV_EVE_GATEWAY_API: &str = "STARFOUNDRY_EVE_GATEWAY_API_URL";

pub struct EveGatewayClient(MtlsApiClient);

impl EveGatewayClient {
    pub fn new(
        service: String,
    ) -> Result<Self> {
        let api_url = Self::api_url()?;
        let mtls_client = MtlsApiClient::new(api_url, service)?;
        Ok(Self(mtls_client))
    }

    fn api_url() -> Result<Url> {
        let env = if let Ok(x) = std::env::var(ENV_EVE_GATEWAY_API) {
            x
        } else {
            return Err(Error::EnvNotSet(ENV_EVE_GATEWAY_API).into());
        };

        Url::parse(&env)
            .map_err(Error::UrlParseError)
            .map_err(Into::into)
    }
}

impl ApiClient for EveGatewayClient {
    async fn fetch<Q:Serialize, T>(
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

impl EveGatewayApiClient for EveGatewayClient {}

impl EveGatewayApiClientAsset for EveGatewayClient {}

impl EveGatewayApiClientContract for EveGatewayClient {}

impl EveGatewayApiClientIndustry for EveGatewayClient {}

impl EveGatewayApiClientItem for EveGatewayClient {}

impl EveGatewayApiClientMarket for EveGatewayClient {}
