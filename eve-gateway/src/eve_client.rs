mod character;
mod corporation;
mod jwt_key;
mod jwt;
mod utils;

/// errors that can be thrown by this module
pub mod error;

pub use self::jwt::*;
pub use self::character::*;
pub use self::corporation::*;

use reqwest::{Client, Response, StatusCode};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use url::Url;

use crate::eve_client::error::{EveApiError, Result};

/// Required by the EVE-Api
const COMPATIBILITY_DATE_HEADER: &str       = "X-Compatibility-Date";
const COMPATIBILITY_DATE_VALUE: HeaderValue = HeaderValue::from_static("2020-01-01");

const ENV_API_URL: &str                     = "STARFOUNDRY_EVE_GATEWAY_EVE_API_URL";
const ENV_OAUTH_AUTHORIZATION_URL: &str     = "STARFOUNDRY_EVE_GATEWAY_EVE_OAUTH_AUTHORIZATION_URL";
const ENV_OAUTH_JWT_KEYS_URL: &str          = "STARFOUNDRY_EVE_GATEWAY_EVE_OAUTH_JWT_KEYS_URL";
const ENV_OAUTH_TOKEN_URL: &str             = "STARFOUNDRY_EVE_GATEWAY_EVE_OAUTH_TOKEN_URL";

const ENV_USER_AGENT: &str                  = "STARFOUNDRY_EVE_GATEWAY_USER_AGENT";

/// Environment variable for setting the client id
pub const ENV_CLIENT_ID: &str               = "STARFOUNDRY_EVE_GATEWAY_EVE_CLIENT_ID";
/// Environment variable for setting the secret key
pub const ENV_SECRET_KEY: &str              = "STARFOUNDRY_EVE_GATEWAY_EVE_SECRET_KEY";
/// Environment variable for setting the callback
pub const ENV_CALLBACK: &str                = "STARFOUNDRY_EVE_GATEWAY_EVE_CALLBACK";

/// Client for communicating with the EVE-API.
/// The client can either do unauthenticated requests or do them with an authed
/// corporation or character.
/// 
/// If an authenticated client is constructed, only the refresh_token is required,
/// an access token will be fetched as needed.
/// 
/// # Required ENV
/// 
/// If not all required ENVs are set, an error will be returned.
/// All values for the ENV can be found
/// [here](https://developers.eveonline.com/applications)
///
/// * `EVE_CALLBACK`   -> Url to callback after authentication
/// * `ÈVE_CLIENT_ID`  -> Client ID of the application
/// * `EVE_SECRET_KEY` -> Secret key of the application
/// 
#[derive(Clone)]
pub struct EveApiClient {
    /// Determines if the client is an authenticated client
    authenticated:      Option<AuthenticatedClient>,

    /// Client for communicating with EVE
    client:             Client,

    /// Token needed to get data that is behind auth
    access_token:       Arc<Mutex<Option<String>>>,
}

impl EveApiClient {
    /// Constructs a new [EveApiClient].
    /// 
    /// The instance will not be authenticated.
    /// Calling routes that require authentication with this client will result
    /// in an error.
    /// 
    /// # Errors
    /// 
    /// - If the reqwest Client cannot be constructed.
    /// - If the ENV '[ENV_CLIENT_ID]' is not set
    /// 
    pub fn new() -> Result<Self> {
        let client = Self::client()?;

        Ok(Self {
            client:        client,
            access_token:  Arc::new(Mutex::new(None)),

            authenticated: None,
        })
    }

    /// Constructs a new authenticated [EveApiClient] from a given refresh token.
    /// The client will get an access_token if necessary of if an already
    /// fetched one is expired.
    /// 
    /// # Errors
    /// 
    /// - If the reqwest client cannot be constructed
    /// 
    pub fn new_with_refresh_token(
        character_id:  CharacterId,
        refresh_token: impl Into<String>,
    ) -> Result<Self> {
        let client = Self::client()?;

        Ok(Self {
            client:         client,

            access_token:   Arc::new(Mutex::new(None)),

            authenticated:  Some(AuthenticatedClient {
                refresh_token: refresh_token.into(),
                character_id,
            }),
        })
    }

    /// Gets the initial access token.
    ///
    /// [More information](https://docs.esi.evetech.net/docs/sso/web_based_sso_flow.html)
    /// 
    /// # Errors
    /// 
    /// - If the retrieving of an `access_token` fails
    /// - If the ENV '[ENV_CLIENT_ID]' is not set
    /// - If the ENV '[ENV_SECRET_KEY]' is not set
    /// - If the ENV '[ENV_OAUTH_TOKEN_URL]' is not set
    ///
    pub async fn access_token(
        code: &str,
    ) -> Result<EveJwtToken> {
        let mut map = HashMap::new();
        map.insert("grant_type", "authorization_code");
        map.insert("code", code);

        Self::get_token(map).await
    }

    /// Generates a url for authenticating a character against the EVE-API.
    /// 
    /// # Params
    /// 
    /// * `state` -> Unique key to allow to identify the user when he comes back
    ///              using the callback route
    /// * `scope` -> Required scope, must be a list of space separated entries
    /// 
    /// # Errors
    /// 
    /// - If the ENV '[ENV_CLIENT_ID]' is not set
    /// - If the ENV '[ENV_CALLBACK]' is not set
    /// - If the ENV '[ENV_OAUTH_AUTHORIZATION_URL]' is not set
    /// 
    pub fn auth_uri(
        state: &str,
        scope: &str
    ) -> Result<Url> {
        let client_id = Self::client_id()?;
        let callback = Self::callback()?;

        let mut url = Self::oauth_authorization_url()?;
        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &callback)
            .append_pair("client_id", &*client_id)
            .append_pair("scope", scope)
            .append_pair("state", state);
        Ok(url)
    }

    /// Makes a single request to the given path and returns parses the result
    /// the given struct.
    /// 
    /// The response will be deserialized into the given model `T`.
    /// 
    /// # Errors
    /// 
    /// - Request fails
    /// - Parsing the response fails
    /// 
    pub async fn fetch<Q: Serialize, T>(
        &self,
        path:  impl Into<String>,
        query: &Q,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut api_url = Self::api_url()?;
        api_url.set_path(&path.into());

        let response = self
            .send(api_url.clone(), query)
            .await?;

        match response.text().await {
            Err(e) => {
                tracing::error!("Error parsing json, {}", e);
                Err(EveApiError::ReqwestError(e, api_url))
            },
            Ok(x) => {
                serde_json::from_str(&x)
                    .map_err(|e| EveApiError::JsonParseError(e, x, api_url))
            }
        }
    }

    /// Makes a single request to the given path and returns parses the result
    /// the given struct.
    ///
    /// # Params
    ///
    /// * `T`    -> Model that represents the resulting json
    /// * `path` -> Path of the request
    ///
    /// # Errors
    ///
    /// Returns an error if either the request failed or the parsing failed
    ///
    /// # Returns
    ///
    /// Parsed json data
    ///
    pub async fn fetch_auth<Q: Serialize, T>(
        &self,
        path:  impl Into<String>,
        query: &Q,
    ) -> Result<T>
    where
        T: DeserializeOwned,
    {
        let mut api_url = Self::api_url()?;
        api_url.set_path(&path.into());

        let response = self
            .send_auth(api_url.clone(), query)
            .await?;

        match response.text().await {
            Err(e) => {
                tracing::error!("Error parsing json, {}", e);
                Err(EveApiError::ReqwestError(e, api_url))
            },
            Ok(x) => {
                serde_json::from_str(&x)
                    .map_err(|e| EveApiError::JsonParseError(e, x, api_url))
            }
        }
    }

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
    pub async fn fetch_page<T>(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<T>>
    where
        T: DeserializeOwned + Send,
    {
        let mut api_url = Self::api_url()?;
        api_url.set_path(&path.into());

        let response = self
            .send(api_url.clone(), &())
            .await?;

        let pages = self.page_count(&response);

        if !self.has_content(&response) {
            return Ok(Vec::new());
        }

        let mut data: Vec<T> = Vec::new();
        let fetched_data = response
            .json::<Vec<T>>()
            .await
            .map_err(|x| EveApiError::ReqwestError(x, api_url.clone()))?;
        data.extend(fetched_data);

        for page in 2..=pages {
            let next_page = self
                .send(
                    api_url.clone(),
                    &[("page", &page.to_string())],
                )
                .await?;

            if !self.has_content(&next_page) {
                continue;
            }

            let next_page = next_page
                .json::<Vec<T>>()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, api_url.clone()))?;
            data.extend(next_page);
        }

        Ok(data)
    }

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
    pub async fn fetch_page_auth<T>(
        &self,
        path: impl Into<String>,
    ) -> Result<Vec<T>>
    where
        T: std::fmt::Debug + DeserializeOwned + Send,
    {
        let mut api_url = Self::api_url()?;
        api_url.set_path(&path.into());

        let response = self
            .send_auth(api_url.clone(), &())
            .await?;

        let pages = self.page_count(&response);

        if !self.has_content(&response) {
            return Ok(Vec::new());
        }

        let mut data: Vec<T> = Vec::new();
        let fetched_data = response
            .json::<Vec<T>>()
            .await
            .map_err(|x| EveApiError::ReqwestError(x, api_url.clone()))?;
        data.extend(fetched_data);

        for page in 2..=pages {
            let next_page = self
                .send_auth(
                    api_url.clone(),
                    &[("page", &page.to_string())],
                )
                .await?;

            if !self.has_content(&next_page) {
                continue;
            }

            let next_page = next_page
                .json::<Vec<T>>()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, api_url.clone()))?;
            data.extend(next_page);
        }

        Ok(data)
    }

    /// Makes a post request to the given path and returns parses the result
    /// the given struct.
    ///
    /// # Params
    ///
    /// * `T`    -> Model that represents the resulting json
    /// * `data` -> Request model
    /// * `path` -> Path of the request
    ///
    /// # Errors
    ///
    /// Returns an error if either the request failed or the parsing failed
    ///
    /// # Returns
    ///
    /// Parsed json data
    ///
    pub async fn post<D, T>(
        &self,
        data: D,
        path: impl Into<String>,
    ) -> Result<T>
    where
        D: Debug + Serialize + Send + Sync,
        T: DeserializeOwned,
    {
        let mut api_url = Self::api_url()?;
        api_url.set_path(&path.into());

        let json = self
            .send_post(data, api_url.clone())
            .await?
            .json::<T>()
            .await
            .map_err(|x| EveApiError::ReqwestError(x, api_url))?;
        Ok(json)
    }

    /// Sends a GET request to the given path setting the current `access_token`
    /// as `bearer_auth`.
    ///
    /// If a request fails with a non successful status, it will retry the
    /// request again, up to 3 times.
    ///
    /// # Params
    ///
    /// * `path` -> Path for the request
    ///
    /// # Errors
    ///
    /// The function errors if too many request failed, or if there is a general
    /// error with the requesting library.
    ///
    /// If the EVE-API returns [StatusCode::UNAUTHORIZED] it will attempt to
    /// retrieve a new `access_token`. If that fails an error is returned.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    async fn send<Q: Serialize>(
        &self,
        request_uri: Url,
        query:       &Q,
    ) -> Result<Response> {
        let mut retry_counter = 0usize;
        let mut last_status   = StatusCode::OK;
        let mut last_text     = String::new();

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", request_uri);
                return Err(EveApiError::TooManyRetries(
                    request_uri,
                    last_status,
                    last_text,
                ));
            }

            let response = self
                .client
                .get(request_uri.clone())
                .query(query)
                .send()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, request_uri.clone()))?;

            if response.headers().get("X-Ratelimit-Group").is_some() {
                let group = response.headers().get("X-Ratelimit-Group").unwrap().to_str().unwrap();
                let remaining: f64 = response.headers().get("X-Ratelimit-Remaining").unwrap().to_str().unwrap().parse::<f64>().unwrap();

                let label = [
                    ("group", group.to_string()),
                ];
                metrics::gauge!("eve-api-rate-limit", &label).set(remaining);
            }

            match response.status() {
                StatusCode::NOT_FOUND => {
                    return Err(EveApiError::NotFound(request_uri));
                },
                StatusCode::TOO_MANY_REQUESTS => {
                    return Err(EveApiError::RateLimit(request_uri));
                },
                StatusCode::NOT_MODIFIED => {
                    return Err(EveApiError::NotModified(request_uri));
                },
                StatusCode::FORBIDDEN |
                StatusCode::UNAUTHORIZED => {
                    let content = response.text().await.unwrap_or_default();
                    return Err(EveApiError::Unauthorized(request_uri, content));
                },
                StatusCode::BAD_GATEWAY => {
                    return Err(EveApiError::BadGateway);
                },
                StatusCode::SERVICE_UNAVAILABLE => {
                    return Err(EveApiError::ServiceUnavailable);
                },
                _ => {}
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

    /// Gets an `access_token` either from the local cache or fetched from the
    /// EVE-API.
    ///
    /// [More information](https://docs.esi.evetech.net/docs/sso/refreshing_access_tokens.html)
    ///
    /// # Errors
    ///
    /// - If the retrieving of an `access_token` fails
    /// 
    async fn get_access_token(
        &self,
    ) -> Result<String> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(EveApiError::ClientNotAuthenticated);
        };

        let mut map = HashMap::new();
        map.insert("grant_type", "refresh_token");
        map.insert("refresh_token", &authenticated.refresh_token);

        let token = Self::get_token(map).await?;

        #[allow(clippy::unwrap_used)]
        {
            *self.access_token.lock().unwrap() = Some(token.access_token.clone());
        }

        Ok(token.access_token)
    }

    /// Sends a GET request to the given path setting the current `access_token`
    /// as `bearer_auth`.
    ///
    /// If a request fails with a non successful status, it will retry the
    /// request again, up to 3 times.
    ///
    /// # Params
    ///
    /// * `path` -> Path for the request
    ///
    /// # Errors
    ///
    /// The function errors if too many request failed, or if there is a general
    /// error with the requesting library.
    ///
    /// If the EVE-API returns [StatusCode::UNAUTHORIZED] it will attempt to
    /// retrieve a new `access_token`. If that fails an error is returned.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    async fn send_auth<Q: Serialize>(
        &self,
        request_uri: Url,
        query:       &Q,
    ) -> Result<Response> {
        let access_token = {
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        };
        let mut access_token = if access_token.is_none() {
            self.get_access_token().await?;
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        } else {
            access_token
        };

        let mut retry_counter = 0usize;
        let mut last_status   = StatusCode::OK;
        let mut last_text     = String::new();

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", request_uri);
                return Err(EveApiError::TooManyRetries(
                    request_uri,
                    last_status,
                    last_text,
                ));
            }

            let token = access_token
                .as_ref()
                .expect("We check but somehow the access_token is still None");
            let response = self
                .client
                .get(request_uri.clone())
                .query(query)
                .bearer_auth(token)
                .send()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, request_uri.clone()))?;

            if response.headers().get("X-Ratelimit-Group").is_some() {
                let group = response.headers().get("X-Ratelimit-Group").unwrap().to_str().unwrap();
                let remaining: f64 = response.headers().get("X-Ratelimit-Remaining").unwrap().to_str().unwrap().parse::<f64>().unwrap();

                let character_id = if let Some(x) = self
                    .authenticated
                    .clone()
                    .map(|x| x.character_id) {

                    x
                } else {
                    CharacterId(0)
                };

                let label = [
                    ("group", group.to_string()),
                    ("character_id", character_id.to_string()),
                ];
                metrics::gauge!("eve-api-rate-limit", &label).set(remaining);
            }

            match response.status() {
                StatusCode::NOT_FOUND => {
                    return Err(EveApiError::NotFound(request_uri));
                },
                StatusCode::TOO_MANY_REQUESTS => {
                    return Err(EveApiError::RateLimit(request_uri));
                },
                StatusCode::NOT_MODIFIED => {
                    return Err(EveApiError::NotModified(request_uri));
                },
                StatusCode::FORBIDDEN |
                StatusCode::UNAUTHORIZED => {
                    last_status = response.status();
                    last_text   = response
                        .text()
                        .await
                        .unwrap_or_default();

                    retry_counter += 1;
                    access_token = Some(self.get_access_token().await?);

                    // Wait a second before trying again
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    continue;
                },
                StatusCode::BAD_GATEWAY => {
                    return Err(EveApiError::BadGateway);
                },
                StatusCode::SERVICE_UNAVAILABLE => {
                    return Err(EveApiError::ServiceUnavailable);
                },
                _ => {}
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

    /// Sends a POST request to the given path setting the current
    /// `access_token` as `bearer_auth`.
    ///
    /// If a request fails with a non successful status, it will retry the
    /// request again, up to 3 times.
    ///
    /// # Params
    ///
    /// * `data` -> Data to send in the body
    /// * `path` -> Path for the request
    ///
    /// # Errors
    ///
    /// The function errors if too many request failed, or if there is a general
    /// error with the requesting library.
    ///
    /// If the EVE-API returns [StatusCode::UNAUTHORIZED] it will attempt to
    /// retrieve a new `access_token`. If that fails an error is returned.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    async fn send_post<R>(
        &self,
        data:        R,
        request_uri: Url,
    ) -> Result<Response>
    where
        R: Debug + Serialize + Send + Sync,
    {
        let access_token = {
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        };
        let mut access_token = if access_token.is_none() {
            self.get_access_token().await?;
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        } else {
            access_token
        };

        let mut retry_counter = 0usize;
        let mut last_status   = StatusCode::OK;
        let mut last_text     = String::new();

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", request_uri);
                return Err(EveApiError::TooManyRetries(
                    request_uri,
                    last_status,
                    last_text,
                ));
            }

            let token = access_token
                .as_ref()
                .expect("We check but somehow the access_token is still None");
            let response = self
                .client
                .post(request_uri.clone())
                .json(&data)
                .bearer_auth(token)
                .send()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, request_uri.clone()))?;

            if response.status() == StatusCode::FORBIDDEN ||
               response.status() == StatusCode::UNAUTHORIZED {

                last_status = response.status();
                last_text   = response
                    .text()
                    .await
                    .unwrap_or_default();

                retry_counter += 1;
                access_token = Some(self.get_access_token().await?);

                // Wait a second before trying again
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
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

    /// Makes a request to the token interface and sets necessary headers to
    /// retrieve a new `access_token`.
    ///
    /// # Params
    ///
    /// * `form` -> Form containing `grant_type` and `code` or `refresh_token`.
    ///             See the EVE SSO-Flow documentation for more information
    ///
    /// # Errors
    ///
    /// If the request fails
    ///
    /// # Returns
    ///
    /// New token object
    ///
    async fn get_token(
        form: HashMap<&str, &str>,
    ) -> Result<EveJwtToken> {
        let client_id = (*Self::client_id()?).clone();
        let secret_key = (*Self::secret_key()?).clone();
        let oauth_token_url = Self::oauth_token_url()?;

        let response = Client::new()
            .post(oauth_token_url.clone())
            .basic_auth(client_id, Some(secret_key))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&form)
            .send()
            .await
            .map_err(EveApiError::GetTokenRequestError)?;

        if response.status().is_success() {
            let token: EveJwtToken = response
                .json()
                .await
                .map_err(|e| EveApiError::ReqwestError(e, oauth_token_url.into()))?;
            token.validate(
                Self::oauth_jwt_keys_url()?,
                Self::client_id()?,
            ).await?;

            Ok(token)
        } else {
            return Err(EveApiError::GetTokenError)
        }
    }

    /// Extract the page header from the give [reqwest::Response].
    ///
    /// # Params
    ///
    /// * `response` -> Response to get the header from
    ///
    /// # Returns
    ///
    /// - If the header is not present a 0 is returned
    /// - If the header exists, it will try to parse it, if that fails a 0 is
    ///   is returned
    ///
    fn page_count(&self, response: &Response) -> u16 {
        let headers = response.headers();
        if let Some(x) = headers.get("x-pages") {
            x.to_str()
                .unwrap_or_default()
                .parse::<u16>()
                .unwrap_or_default()
        } else {
            0u16
        }
    }

    /// Checks if the return has content in it.
    /// 
    /// # Params
    ///
    /// * `response` -> Response to get the header from
    ///
    /// # Returns
    ///
    /// - False if the header does not exists
    /// - False if the content does not contain data
    ///   True if the content contains data
    ///
    fn has_content(
        &self,
        response: &Response
    ) -> bool {
        let headers = response.headers();
        if let Some(x) = headers.get("content-length") {
            x.to_str()
                .unwrap_or_default()
                .parse::<u64>()
                .unwrap_or_default() > 0
        } else {
            false
        }
    }

    fn client() -> Result<Client> {
        let mut headers = HeaderMap::new();
        headers.insert(
            COMPATIBILITY_DATE_HEADER,
            COMPATIBILITY_DATE_VALUE,
        );

        Client::builder()
            .user_agent(Self::user_agent())
            .default_headers(headers)
            .https_only(true)
            .build()
            .map_err(EveApiError::CouldNotConstructClient)
    }

    /// Reads the EVE API URL from the environment `STARFOUNDRY_EVE_CLIENT_API_URL
    /// 
    pub fn api_url() -> Result<Url> {
        std::env::var(ENV_API_URL)
            .map(|x| Url::parse(&x).map_err(EveApiError::UrlParseError))
            .unwrap_or(Url::parse("https://esi.evetech.net").map_err(EveApiError::UrlParseError))
    }

    /// Reads the EVE OAuth authorization URL from the environment
    /// `STARFOUNDRY_EVE_CLIENT_OAUTH_AUTHORIZATION_URL
    /// 
    pub fn oauth_authorization_url() -> Result<Url> {
        std::env::var(ENV_OAUTH_AUTHORIZATION_URL)
            .map(|x| Url::parse(&x).map_err(EveApiError::UrlParseError))
            .unwrap_or(Url::parse("https://login.eveonline.com/v2/oauth/authorize").map_err(EveApiError::UrlParseError))
    }

    /// Reads the EVE jwt key URL from the environment
    /// `STARFOUNDRY_EVE_CLIENT_OAUTH_JWT_KEYS_URL
    /// 
    pub fn oauth_jwt_keys_url() -> Result<Url> {
        std::env::var(ENV_OAUTH_JWT_KEYS_URL)
            .map(|x| Url::parse(&x).map_err(EveApiError::UrlParseError))
            .unwrap_or(Url::parse("https://login.eveonline.com/oauth/jwks").map_err(EveApiError::UrlParseError))
    }

    /// Reads the EVE oauth token URL from the environment
    /// `STARFOUNDRY_EVE_CLIENT_OAUTH_TOKEN_URL
    /// 
    pub fn oauth_token_url() -> Result<Url> {
        std::env::var(ENV_OAUTH_TOKEN_URL)
            .map(|x| Url::parse(&x).map_err(EveApiError::UrlParseError))
            .unwrap_or(Url::parse("https://login.eveonline.com/v2/oauth/token").map_err(EveApiError::UrlParseError))
    }

    fn user_agent() -> String {
        std::env::var(ENV_USER_AGENT).unwrap_or(String::from("StarFoundry"))
    }

    /// Gets the client_id from the environment `STARFOUNDRY_EVE_CLIENT_ID`
    /// 
    pub fn client_id() -> Result<EveClientId> {
        std::env::var(ENV_CLIENT_ID)
            .map(|x| EveClientId(x))
            .map_err(|_| EveApiError::EnvNotSet(ENV_CLIENT_ID))
    }

    fn secret_key() -> Result<EveSecretKey> {
        std::env::var(ENV_SECRET_KEY)
            .map(|x| EveSecretKey(x))
            .map_err(|_| EveApiError::EnvNotSet(ENV_SECRET_KEY))
    }

    fn callback() -> Result<String> {
        std::env::var(ENV_CALLBACK)
            .map_err(|_| EveApiError::EnvNotSet(ENV_CALLBACK))
    }
}

/// Holds the information about an authenticated character
#[derive(Clone, Debug)]
pub struct AuthenticatedClient {
    /// [CharacterId] of the character this authenticated client belongs to
    pub character_id:   CharacterId,

    refresh_token:       String,
}

/// Wrapper struct the the ClientId, given by CCP
#[derive(Clone)]
pub struct EveClientId(String);

impl Deref for EveClientId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for EveClientId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Wrapper struct the the SecretKey, given by CCP
#[derive(Clone)]
pub struct EveSecretKey(String);

impl Deref for EveSecretKey {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for EveSecretKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "****")
    }
}
