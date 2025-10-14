use std::fmt::Debug;

use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_lib_gateway::{ApiClient, MtlsApiClient, Result as GatewayResult};
use url::Url;

use crate::error::{Error, Result};
use crate::EveGatewayApiClient;

pub const ENV_EVE_GATEWAY_API: &str = "STARFOUNDRY_EVE_GATEWAY_API_URL";

pub struct EveGatewayClient(MtlsApiClient);

impl EveGatewayClient {
    pub fn new() -> Result<Self> {
        let api_url = Self::api_url()?;
        let mtls_client = MtlsApiClient::new(api_url)?;
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
    async fn fetch<T>(
        &self,
        path: impl Into<String>,
    ) -> GatewayResult<T>
    where
        T: DeserializeOwned {

        self.0
            .fetch(path)
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
