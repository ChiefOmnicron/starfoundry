use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_lib_gateway::{ApiClient, Identity, Result as GatewayResult, StarFoundryApiClient};
use url::Url;

use crate::error::{Error, Result};
use crate::{ENV_EVE_GATEWAY_API, EveGatewayApiClient, EveGatewayApiClientAsset, EveGatewayApiClientEveAsset, EveGatewayApiClientFitting, EveGatewayApiClientIndustry, EveGatewayApiClientItem, EveGatewayApiClientSearch};
use crate::contract::EveGatewayApiClientContract;
use crate::market::EveGatewayApiClientMarket;

pub struct EveGatewayClient(StarFoundryApiClient);

impl EveGatewayClient {
    pub fn new<S: Into<String>>(
        service: S,
    ) -> Result<Self> {
        let api_url = Self::api_url()?;
        let api_client = StarFoundryApiClient::new(api_url, service.into())?;
        Ok(Self(api_client))
    }

    pub fn new_with_identity<S: Into<String>>(
        service:    S,
        identity:   Identity,
    ) -> Result<Self> {
        let api_url = Self::api_url()?;
        let api_client = StarFoundryApiClient::new_with_identity(
            api_url,
            service.into(),
            identity,
        )?;
        Ok(Self(api_client))
    }

    fn api_url() -> Result<Url> {
        let env = if let Ok(x) = std::env::var(ENV_EVE_GATEWAY_API) {
            x
        } else {
            return Err(Error::EnvNotSet(ENV_EVE_GATEWAY_API));
        };

        Url::parse(&env)
            .map_err(Error::UrlParseError)
    }
}

impl ApiClient for EveGatewayClient {
    async fn fetch<Q:Serialize, T>(
        &self,
        path:  impl Into<String>,
        query: &Q,
    ) -> GatewayResult<T>
    where
        T: Default + DeserializeOwned {

        self.0
            .fetch(path, query)
            .await
    }

    async fn fetch_auth<Q: Serialize, T>(
        &self,
        path:  impl Into<String>,
        query: &Q,
    ) -> GatewayResult<T>
    where
        T: Default + DeserializeOwned {

        self.0
            .fetch_auth(path, query)
            .await
    }

    async fn post<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> GatewayResult<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: Default + DeserializeOwned {

        self.0
            .post(path, data)
            .await
    }

    async fn post_auth<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> GatewayResult<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: Default + DeserializeOwned {

        self.0
            .post_auth(path, data)
            .await
    }

    async fn put_auth<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> GatewayResult<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: Default + DeserializeOwned {

        self.0
            .put_auth(path, data)
            .await
    }

    async fn delete_auth<T>(
        &self,
        path: impl Into<String>,
    ) -> GatewayResult<T>
    where
        T: Default + DeserializeOwned {

        self.0
            .delete_auth(path)
            .await
    }
}

impl EveGatewayApiClient for EveGatewayClient {}
impl EveGatewayApiClientAsset for EveGatewayClient {}
impl EveGatewayApiClientContract for EveGatewayClient {}
impl EveGatewayApiClientEveAsset for EveGatewayClient {}
impl EveGatewayApiClientFitting for EveGatewayClient {}
impl EveGatewayApiClientMarket for EveGatewayClient {}
impl EveGatewayApiClientIndustry for EveGatewayClient {}
impl EveGatewayApiClientItem for EveGatewayClient {}
impl EveGatewayApiClientSearch for EveGatewayClient {}
