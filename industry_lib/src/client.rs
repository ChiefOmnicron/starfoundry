use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_lib_gateway::{ApiClient, Identity, Result as GatewayResult, StarFoundryApiClient};
use url::Url;

use crate::error::{Error, Result};
use crate::industry::IndustryApiClientIndustry;
use crate::project::IndustryApiClientProject;
use crate::tag::IndustryApiClientTag;

pub const ENV_INDUSTRY_API: &str = "STARFOUNDRY_INDUSTRY_API_URL";

pub struct IndustryClient(StarFoundryApiClient);

impl IndustryClient {
    /// Creates a new [IndustryClient] pulling the address form the ENV
    /// ENV: `STARFOUNDRY_INDUSTRY_API_URL`
    /// 
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

    /// Creates a new [IndustryClient] using the given address
    /// 
    pub fn new_with_address(
        service: String,
        address: String,
    ) -> Result<Self> {
        let api_url = Url::parse(&address).map_err(Error::UrlParseError)?;

        let api_client = StarFoundryApiClient::new(api_url, service)?;
        Ok(Self(api_client))
    }

    fn api_url() -> Result<Url> {
        let env = if let Ok(x) = std::env::var(ENV_INDUSTRY_API) {
            x
        } else {
            return Err(Error::EnvNotSet(ENV_INDUSTRY_API).into());
        };

        Url::parse(&env)
            .map_err(Error::UrlParseError)
            .map_err(Into::into)
    }
}

impl ApiClient for IndustryClient {
    async fn fetch<Q: Serialize, T>(
        &self,
        path:  impl Into<String>,
        query: &Q,
    ) -> GatewayResult<T>
    where
        T: Default + DeserializeOwned {

        self.0
            .fetch(path, query)
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
        T: Default + DeserializeOwned {

        self.0
            .post(path, data)
            .await
            .map_err(Into::into)
    }

    async fn put<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> GatewayResult<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: Default + DeserializeOwned {

        self.0
            .put(path, data)
            .await
            .map_err(Into::into)
    }

    async fn delete<T>(
        &self,
        path: impl Into<String>,
    ) -> GatewayResult<T>
    where
        T: Default + DeserializeOwned {

        self.0
            .delete(path)
            .await
            .map_err(Into::into)
    }
}

impl IndustryApiClient for IndustryClient {}
impl IndustryApiClientIndustry for IndustryClient {}
impl IndustryApiClientProject for IndustryClient {}
impl IndustryApiClientTag for IndustryClient {}

/// Trait that should be implemented on all clients
/// The default implementation will be sufficient in most cases, overwriting
/// them is only recommended for mocking tests
pub trait IndustryApiClient:
    ApiClient +
    IndustryApiClientIndustry +
    IndustryApiClientProject +
    IndustryApiClientTag {}
