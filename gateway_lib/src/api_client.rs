use axum::http::{HeaderMap, HeaderValue};
use reqwest::{Client, Method, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;
use url::Url;

use crate::error::{Error, Result};
use crate::HEADER_SERVICE;

pub const ENV_MARKET_API: &str = "STARFOUNDRY_MARKET_API_URL";

pub const ENV_EVE_GATEWAY_API: &str      = "STARFOUNDRY_EVE_GATEWAY_API_URL";
pub const ENV_EVE_GATEWAY_JWT_SIGN: &str = "STARFOUNDRY_EVE_GATEWAY_JWT_SIGN";

pub const ENV_USER_AGENT: &str = "STARFOUNDRY_USER_AGENT";

const HEADER_SERVICE_UNKNOWN: &str = "Unknown";

#[derive(Clone)]
pub struct StarFoundryApiClient {
    address: Url,
    client:  Client,
}

impl StarFoundryApiClient {
    pub fn new<S: Into<String>>(
        address: Url,
        service: S,
    ) -> Result<Self> {
        let client = Self::new_raw(service)?;
        Ok(Self {
            address,
            client,
        })
    }

    /// Creates a new [reqwest::Client] without pre-defined address
    /// 
    pub fn new_raw<S: Into<String>>(
        service: S,
    ) -> Result<Client> {
        let mut headers = HeaderMap::new();
        headers.insert(
            HEADER_SERVICE,
            HeaderValue::from_str(
                &service.into()
            ).unwrap_or(
                HeaderValue::from_static(HEADER_SERVICE_UNKNOWN)
            )
        );

        Client::builder()
            .user_agent(Self::user_agent()?)
            .default_headers(headers)
            .build()
            .map_err(Error::CouldNotConstructClient)
            .map_err(Into::into)
    }

    pub async fn fetch<Q: Serialize, T>(
        &self,
        path:  impl Into<String>,
        query: &Q,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        self.fetch_auth(
                path,
                query,
                HeaderMap::new(),
            )
            .await
    }

    pub async fn fetch_auth<Q: Serialize, T>(
        &self,
        path:    impl Into<String>,
        query:   &Q,
        headers: HeaderMap,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut api_url = self.address.clone();
        api_url.set_path(&path.into());

        let response = self
            .send(
                Method::GET,
                api_url.clone(),
                serde_json::Value::Null,
                query,
                Some(headers),
            )
            .await?;

        match response.text().await {
            Err(e) => {
                tracing::error!("Error parsing json, {}", e);
                Err(Error::ReqwestError(e, api_url))
            },
            Ok(x) => {
                serde_json::from_str(&x)
                    .map_err(|e| Error::JsonParseError(e, x, api_url))
            }
        }
    }

    pub async fn post<D, T>(
        &self,
        path: impl Into<String>,
        data: D,
    ) -> Result<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: Default + DeserializeOwned,
    {
        let path = path.into();

        let mut api_url = self.address.clone();
        api_url.set_path(&path.clone());

        let response = self
            .send(
                Method::POST,
                api_url.clone(),
                serde_json::to_value(&data)?,
                &(),
                None,
            )
            .await?;

        match response.status() {
            StatusCode::NOT_FOUND => {
                return Err(Error::NotFound(api_url).into());
            },
            StatusCode::FORBIDDEN => {
                return Err(Error::Forbidden(api_url).into());
            },
            StatusCode::UNAUTHORIZED => {
                return Err(Error::Unauthorized.into());
            },
            StatusCode::BAD_GATEWAY => {
                return Err(Error::BadGateway.into());
            },
            StatusCode::SERVICE_UNAVAILABLE => {
                return Err(Error::ServiceUnavailable.into());
            },
            StatusCode::NO_CONTENT => {
                return Ok(T::default());
            },
            StatusCode::OK => {
                return response
                    .json::<T>()
                    .await
                    .map_err(|x| Error::ReqwestError(x, api_url));
            },
            _ => {
                // TODO: better fallback
                return Err(Error::Unauthorized.into());
            }
        };
    }

    fn user_agent() -> Result<String> {
        std::env::var(ENV_USER_AGENT)
            .map_err(|_| Error::EnvNotSet(ENV_USER_AGENT))
            .map_err(Into::into)
    }

    async fn send<Q: Serialize>(
        &self,
        method:  Method,
        url:     Url,
        body:    serde_json::Value,
        query:   &Q,
        headers: Option<HeaderMap>,
    ) -> Result<Response> {
        let mut retry_counter = 0usize;
        let mut last_status   = StatusCode::OK;
        let mut last_text     = String::new();

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", url);
                return Err(Error::TooManyRetries(
                    url,
                    last_status,
                    last_text,
                ).into());
            }

            let client = self.client
                .request(method.clone(), url.clone())
                .query(query);

            let client = if body != serde_json::Value::Null {
                client.json(&body)
            } else {
                client
            };

            let client = if let Some(ref x) = headers {
                client.headers(x.clone())
            } else {
                client
            };

            let response = client
                .send()
                .await
                .map_err(|x| Error::ReqwestError(x, url.clone()))?;

            match response.status() {
                StatusCode::NOT_FOUND => {
                    return Err(Error::NotFound(url).into());
                },
                StatusCode::FORBIDDEN => {
                    return Err(Error::Forbidden(url).into());
                },
                StatusCode::UNAUTHORIZED => {
                    return Err(Error::Unauthorized.into());
                },
                StatusCode::BAD_GATEWAY => {
                    return Err(Error::BadGateway.into());
                },
                StatusCode::SERVICE_UNAVAILABLE => {
                    return Err(Error::ServiceUnavailable.into());
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
                        uri = url.as_str(),
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
}

pub trait ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch<Q: Serialize, T>(
        &self,
        path:  impl Into<String>,
        query: &Q,
    ) -> Result<T>
    where
        T: DeserializeOwned;

    #[allow(async_fn_in_trait)]
    async fn fetch_auth<Q: Serialize, T>(
        &self,
        path:    impl Into<String>,
        query:   &Q,
        headers: HeaderMap,
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
        T: Default + DeserializeOwned;
}
