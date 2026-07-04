use reqwest::Method;
use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_lib_gateway::{ApiClient, Identity, Result as GatewayResult, StarFoundryApiClient};
use std::fmt::Debug;
use url::Url;

use crate::error::{Error, Result};
use crate::{ApiClientExtended, ENV_EVE_GATEWAY_API, EveGatewayApiClient, EveGatewayApiClientAsset, EveGatewayApiClientEveAsset, EveGatewayApiClientFitting, EveGatewayApiClientIndustry, EveGatewayApiClientItem, EveGatewayApiClientSearch};
use crate::contract::EveGatewayApiClientContract;
use crate::market::EveGatewayApiClientMarket;
use crate::utils::{has_content, page_count};

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
    }
}

impl ApiClientExtended for EveGatewayClient {
    /// Makes requests as long as there are pages to fetch.
    ///
    /// # Params
    ///
    /// * `T`    -> Model that represents the resulting json
    /// * `path` -> Path of the request
    ///
    /// # Errors
    ///
    /// Returns an error if either the request failed or the parsing failed.
    /// The error is returned the first time an error is encountered.
    ///
    /// # Returns
    ///
    /// Vector of parsed json
    ///
    async fn fetch_page<T>(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned + Send,
    {
        let headers = if let Ok(x) = self.0.identity_as_header() {
            Some(x)
        } else {
            None
        };

        let mut api_url = Self::api_url()?;
        api_url.set_path(&path.into());

        let response = self.0
            .send_raw(
                Method::GET,
                api_url.clone(),
                serde_json::Value::Null,
                &(),
                headers.clone(),
            )
            .await?;

        let pages = page_count(&response);

        if !has_content(&response) {
            return Ok(Vec::new());
        }

        let mut data: Vec<T> = Vec::new();
        let fetched_data = response
            .json::<Vec<T>>()
            .await
            .map_err(Error::ReqwestError)?;
        data.extend(fetched_data);

        for page in 2..=pages {
            let next_page = self.0
                .send_raw(
                    Method::GET,
                    api_url.clone(),
                    serde_json::Value::Null,
                    &[("page", &page.to_string())],
                    headers.clone(),
                )
                .await?;

            if !has_content(&next_page) {
                continue;
            }

            let next_page = next_page
                .json::<Vec<T>>()
                .await
                .map_err(Error::ReqwestError)?;
            data.extend(next_page);
        }

        Ok(data)
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
