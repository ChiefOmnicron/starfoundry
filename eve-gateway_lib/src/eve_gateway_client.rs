pub mod error;

use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use url::Url;

use crate::Result;
use crate::eve_gateway_client::error::EveGatewayClientError;
use reqwest::header::AUTHORIZATION;
use axum::http::{HeaderMap, HeaderValue};

pub const ENV_EVE_GATEWAY_API: &str = "STARFOUNDRY_EVE_GATEWAY_API";
pub const ENV_EVE_GATEWAY_JWT_SIGN: &str = "STARFOUNDRY_EVE_GATEWAY_JWT_SIGN";

// either a full chain or the intermediate ca
pub const ENV_MTLS_ROOT_CA: &str    = "STARFOUNDRY_MTLS_ROOT_CA";
pub const ENV_MTLS_IDENTITY: &str   = "STARFOUNDRY_MTLS_IDENTITY";
pub const ENV_USER_AGENT: &str      = "STARFOUNDRY_USER_AGENT";

pub struct GatewayClient(Client);

impl GatewayClient {
    pub fn new(
        access_token: Option<String>,
    ) -> Result<Self> {
        let client = Self::client(access_token)?;

        Ok(Self(client))
    }

    pub fn raw_unauthorized_client() -> Result<Client> {
        let client = Self::client(None)?;

        Ok(client)
    }

    async fn send(
        &self,
        request_uri: Url,
        query:       &[(&str, &str)],
    ) -> Result<Response> {
        let mut retry_counter = 0usize;
        let mut last_status   = StatusCode::OK;
        let mut last_text     = String::new();

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", request_uri);
                return Err(EveGatewayClientError::TooManyRetries(
                    request_uri,
                    last_status,
                    last_text,
                ).into());
            }

            let response = self.0
                .get(request_uri.clone())
                .query(query)
                .send()
                .await
                .map_err(|x| EveGatewayClientError::ReqwestError(x, request_uri.clone()))?;

            match response.status() {
                StatusCode::NOT_FOUND => {
                    return Err(EveGatewayClientError::NotFound(request_uri).into());
                },
                StatusCode::IM_A_TEAPOT => {
                    return Err(EveGatewayClientError::RateLimit(request_uri).into());
                },
                StatusCode::FORBIDDEN => {
                    return Err(EveGatewayClientError::Forbidden(request_uri).into());
                },
                StatusCode::UNAUTHORIZED => {
                    return Err(EveGatewayClientError::Unauthorized.into());
                },
                StatusCode::BAD_GATEWAY => {
                    return Err(EveGatewayClientError::BadGateway.into());
                },
                StatusCode::SERVICE_UNAVAILABLE => {
                    return Err(EveGatewayClientError::ServiceUnavailable.into());
                },
                _ => ()
            };

            let response_status = response.status();
            if !response_status.is_success() {
                last_status = response_status;
                last_text   = response
                    .text()
                    .await
                    .unwrap_or_default();

                retry_counter += 1;
                tracing::error!(
                    {
                        retry = retry_counter,
                        status = response_status.as_u16(),
                        uri = request_uri.as_str(),
                        last_text = last_text,
                    },
                    "Fetch resulted in non successful status code.",
                );

                // Wait a second before trying again
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }

            return Ok(response);
        }
    }

    async fn send_post<R>(
        &self,
        data:        R,
        request_uri: Url,
    ) -> Result<Response>
    where
        R: Debug + Serialize + Send + Sync,
    {
        let mut retry_counter = 0usize;
        let mut last_status   = StatusCode::OK;
        let mut last_text     = String::new();

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", request_uri);
                return Err(EveGatewayClientError::TooManyRetries(
                    request_uri,
                    last_status,
                    last_text,
                ).into());
            }

            let response = self.0
                .post(request_uri.clone())
                .json(&data)
                .send()
                .await
                .map_err(|x| EveGatewayClientError::ReqwestError(x, request_uri.clone()))?;

            if response.status() == StatusCode::FORBIDDEN {
                return Err(EveGatewayClientError::Forbidden(request_uri).into());
            } else if response.status() == StatusCode::UNAUTHORIZED {
                return Err(EveGatewayClientError::Unauthorized.into());
            }

            let response_status = response.status();
            if !response_status.is_success() {
                last_status = response.status();
                last_text   = response
                    .text()
                    .await
                    .unwrap_or_default();

                retry_counter += 1;
                tracing::error!(
                    {
                        retry = retry_counter,
                        status = response_status.as_u16(),
                        uri = request_uri.as_str(),
                        last_text = last_text,
                    },
                    "Fetch resulted in non successful status code.",
                );
                continue;
            }

            return Ok(response);
        }
    }

    fn client(
        access_token: Option<String>,
    ) -> Result<Client> {
        let root_ca = reqwest::Certificate::from_pem(
            Self::root_ca()?.as_bytes()
        )
        .map_err(EveGatewayClientError::GenericReqwestError)?;

        let identity = reqwest::Identity::from_pem(
            Self::identity()?.as_bytes()
        )
        .map_err(EveGatewayClientError::GenericReqwestError)?;

        let client = Client::builder()
            .tls_built_in_root_certs(false)
            .add_root_certificate(root_ca)
            .use_rustls_tls()
            .identity(identity)
            .user_agent(Self::user_agent()?)
            .https_only(true);

        let client = if let Some(x) = access_token {
            let mut headers = HeaderMap::new();
            headers.insert(
                AUTHORIZATION,
                HeaderValue::from_str(&x).unwrap_or(HeaderValue::from_static("")),
            );

            client.default_headers(headers)
        } else {
            client
        };

        client
            .build()
            .map_err(EveGatewayClientError::CouldNotConstructClient)
            .map_err(Into::into)
    }

    fn api_url(&self) -> Result<Url> {
        let env = if let Ok(x) = std::env::var(ENV_EVE_GATEWAY_API) {
            x
        } else {
            return Err(EveGatewayClientError::EnvNotSet(ENV_EVE_GATEWAY_API).into());
        };

        Url::parse(&env)
            .map_err(EveGatewayClientError::UrlParseError)
            .map_err(Into::into)
    }

    fn user_agent() -> Result<String> {
        std::env::var(ENV_USER_AGENT)
            .map_err(|_| EveGatewayClientError::EnvNotSet(ENV_USER_AGENT))
            .map_err(Into::into)
    }

    fn root_ca() -> Result<String> {
        std::env::var(ENV_MTLS_ROOT_CA)
            .map_err(|_| EveGatewayClientError::EnvNotSet(ENV_MTLS_ROOT_CA))
            .map_err(Into::into)
    }

    fn identity() -> Result<String> {
        std::env::var(ENV_MTLS_IDENTITY)
            .map_err(|_| EveGatewayClientError::EnvNotSet(ENV_MTLS_IDENTITY))
            .map_err(Into::into)
    }
}

impl EveGatewayClient for GatewayClient {
    async fn fetch<T>(
        &self,
        path: impl Into<String>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut api_url = self.api_url()?;
        api_url.set_path(&path.into());

        let response = self
            .send(api_url.clone(), &[])
            .await?;

        let data = response
            .json::<T>()
            .await
            .map_err(|x| EveGatewayClientError::ReqwestError(x, api_url))?;
        Ok(data)
    }

    async fn fetch_auth<T>(
        &self,
        path: impl Into<String>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut api_url = self.api_url()?;
        api_url.set_path(&path.into());

        let response = self
            .send(api_url.clone(), &[])
            .await?;

        let data = response
            .json::<T>()
            .await
            .map_err(|x| EveGatewayClientError::ReqwestError(x, api_url))?;
        Ok(data)
    }

    async fn post<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> Result<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: DeserializeOwned,
    {
        let mut api_url = self.api_url()?;
        api_url.set_path(&path.into());

        let json = self
            .send_post(data, api_url.clone())
            .await?
            .json::<T>()
            .await
            .map_err(|x| EveGatewayClientError::ReqwestError(x, api_url))?;
        Ok(json)
    }
}

pub trait EveGatewayClient {
    #[allow(async_fn_in_trait)]
    async fn fetch<T>(
        &self,
        path: impl Into<String>,
    ) -> Result<T>
    where
        T: DeserializeOwned;

    #[allow(async_fn_in_trait)]
    async fn fetch_auth<T>(
        &self,
        path: impl Into<String>,
    ) -> Result<T>
    where
        T: DeserializeOwned;

    #[allow(async_fn_in_trait)]
    async fn post<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> Result<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: DeserializeOwned;
}
