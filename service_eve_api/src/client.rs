mod config;
mod error;
mod jwt;
mod jwt_key;

use chrono::{NaiveDateTime, Utc};
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_libs_types::{CorporationId, CharacterId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use url::Url;

pub use self::config::*;
pub use self::error::*;
pub use self::jwt::*;

const ENV_API_URL: &str                 = "STARFOUNDRY_EVE_CLIENT_API_URL";
const ENV_OAUTH_AUTHORIZATION_URL: &str = "STARFOUNDRY_EVE_CLIENT_OAUTH_AUTHORIZATION_URL";
const ENV_OAUTH_JWT_KEYS_URL: &str      = "STARFOUNDRY_EVE_CLIENT_OAUTH_JWT_KEYS_URL";
const ENV_OAUTH_TOKEN_URL: &str         = "STARFOUNDRY_EVE_CLIENT_OAUTH_TOKEN_URL";

const ENV_USER_AGENT: &str              = "STARFOUNDRY_EVE_USER_AGENT";

/// Client for communicating with the EVE-API using an authenticated character.
///
/// After constructing it provides helper functions for performing a
/// character authentication against the EVE-API.
///
/// # Usage
///
/// Every application should only create a single instance of this struct.
///
/// The client takes the `refresh_token` provided by the EVE-API after login
/// and takes care that there is a valid `access_token`.
///
/// # Required ENV
///
/// If not all required ENVs are set, an error will be returned.
/// All values for the ENV can be found
/// [here](https://developers.eveonline.com/applications)
///
/// * `EVE_USER_AGENT` -> Name of the user agent that is send with every request
/// * `EVE_CALLBACK`   -> Url to callback after authentication
/// * `ÈVE_CLIENT_ID`  -> Client ID of the application
/// * `EVE_SECRET_KEY` -> Secret key of the application
/// 
#[derive(Clone)]
pub struct EveApiClient {
    /// Determines if the client is an authenticated client
    authenticated:  Option<AuthenticatedClient>,

    /// Client for communicating with EVE
    client:             Client,

    /// Token needed to get data that is behind auth
    access_token:       Arc<Mutex<Option<String>>>,
    /// Token needed to get data that is behind auth
    cached_route:       Arc<Mutex<HashMap<String, CachedRouteInfo>>>,
}

impl EveApiClient {
    const COMPATIBILITY_DATE_HEADER: &str       = "X-Compatibility-Date";
    const COMPATIBILITY_DATE_VALUE: HeaderValue = HeaderValue::from_static("2020-01-01");

    /// Gets the initial access token,
    ///
    /// [More information](https://docs.esi.evetech.net/docs/sso/web_based_sso_flow.html)
    ///
    /// # Params
    ///
    /// * `code` -> Code send by the EVE-API as query parameter
    ///
    /// # Panics
    ///
    /// Panics if the [Mutex] is not exclusive.
    ///
    /// # Errors
    ///
    /// If the retrieving of an `access_token` fails the function will return
    /// an error
    ///
    pub async fn access_token(
        client_id:   EveClientId,
        secret_key:  EveSecretKey,
        oauth_token: Url,
        code:   &str,
    ) -> Result<EveJwtToken, EveApiError> {
        let mut map = HashMap::new();
        map.insert("grant_type", "authorization_code");
        map.insert("code", code);

        Self::get_token(
            client_id,
            secret_key,
            oauth_token,
            map,
        ).await
    }

    /// Constructs a new [EveApiClient].
    ///
    /// # Requirements
    ///
    /// The ENV `EVE_USER_AGENT` must be set.
    ///
    /// # Params
    ///
    /// * `refresh_token` -> Refresh token from the EVE-API
    ///
    /// # Errors
    ///
    /// The function will return an error if the ENV `EVE_USER_AGENT` is not set.
    /// Besides that it will return an error if the client could not be
    /// constructed.
    /// See the returned error for more details.
    ///
    /// # Returns
    ///
    /// New instance of the [EveApiClient]
    ///
    pub fn new() -> Result<Self> {
        let client = Self::client()?;

        Ok(Self {
            client:        client,
            access_token:  Arc::new(Mutex::new(None)),
            cached_route:  Arc::new(Mutex::new(HashMap::new())),

            authenticated: None,
        })
    }

    /// Constructs a new [EveApiClient] from a given refresh token
    ///
    /// # Requirements
    ///
    /// The ENV `EVE_USER_AGENT` must be set.
    ///
    /// # Params
    ///
    /// * `refresh_token` -> Refresh token from the EVE-API
    ///
    /// # Errors
    ///
    /// The function will return an error if the ENV `EVE_USER_AGENT` is not set.
    /// Besides that it will return an error if the client could not be
    /// constructed.
    /// See the returned error for more details.
    ///
    /// # Returns
    ///
    /// New instance of the [EveApiClient]
    ///
    pub fn new_with_refresh_token<S>(
        // TODO: validate
        character_id:   CharacterId,
        // TODO: validate
        corporation_id: CorporationId,
        refresh_token:  S,
    ) -> Result<Self, EveApiError>
        where
            S: Into<String>
    {
        let client = Self::client()?;

        Ok(Self {
            client:         client,

            client_id:      config.client_id,
            secret_key:     config.secret_key,

            access_token:   Arc::new(Mutex::new(None)),
            cached_route:   Arc::new(Mutex::new(HashMap::new())),

            authenticated:  Some(AuthenticatedClient {
                refresh_token: refresh_token.into(),
                character_id,
                corporation_id
            }),
        })
    }

    /// Constructs a new [EveApiClient] with an existing `access_token`.
    ///
    /// # Requirements
    ///
    /// The ENV `EVE_USER_AGENT` must be set.
    ///
    /// # Params
    ///
    /// * `access_token`  -> Access token from the EVE-API
    /// * `refresh_token` -> Refresh token from the EVE-API
    ///
    /// # Panics
    ///
    /// Panics if the [Mutex] is not exclusive.
    ///
    /// # Errors
    ///
    /// The function will return an error if the ENV `EVE_USER_AGENT` is not set.
    /// Besides that it will return an error if the client could not be
    /// constructed.
    /// See the returned error for more details.
    ///
    /// # Returns
    ///
    /// New instance of the [EveApiClient]
    ///
    #[allow(clippy::unwrap_in_result)]
    pub fn with_access_token<S>(
        config:         ConfigEveApi,

        character_id:   CharacterId,
        corporation_id: CorporationId,

        access_token:   S,
        refresh_token:  S,
    ) -> Result<Self, EveApiError>
        where
            S: Into<String> {

        let s = Self::new_with_refresh_token(
            config,
            character_id,
            corporation_id,
            refresh_token.into(),
        )?;
        #[allow(clippy::unwrap_used)]
        {
            *s.access_token.lock().unwrap() = Some(access_token.into());
        } 
        Ok(s)
    }

    /// Generates a url for authenticating a character against the EVE-API.
    ///
    /// # Params
    ///
    /// * `state` -> Unique key, used for extra security
    /// * `scope` -> Required scope, must be a lost of space separated entries
    ///
    /// # Errors
    ///
    /// The function will return an error if either the ENV `EVE_CALLBACK`,
    /// the ENV `EVE_CLIENT_ID` or ENV `EVE_CALLBACK` are not set.
    ///
    /// # Usage
    ///
    /// ``` rust
    /// use starfoundry_libs_eve_api::*;
    /// # unsafe {
    /// # std::env::set_var("EVE_CALLBACK", "");
    /// # std::env::set_var("EVE_CLIENT_ID", "");
    /// # std::env::set_var("EVE_SECRET_KEY", "");
    /// # }
    ///
    /// let state = "my_unique_state";
    /// let scope = "publicData esi-industry.read_character_jobs.v1";
    /// let url = EveApiClient::auth_uri(state, scope).unwrap();
    ///
    /// // redirect user to the returned url
    /// ```
    ///
    pub fn auth_uri(
        client_id:           EveClientId,
        callback:            String,
        oauth_authorization: Url,
        state:               &str,
        scope:               &str
    ) -> Result<Url, EveApiError> {
        let mut url = oauth_authorization;

        url.query_pairs_mut()
            .append_pair("response_type", "code")
            .append_pair("redirect_uri", &callback)
            .append_pair("client_id", &client_id)
            .append_pair("scope", scope)
            .append_pair("state", state);
        Ok(url)
    }

    /// Gets a new `access_token` using the `refresh_token`.
    ///
    /// [More information](https://docs.esi.evetech.net/docs/sso/refreshing_access_tokens.html)
    ///
    /// # Errors
    ///
    /// If the retrieving of an `access_token` fails the function will return
    /// an error
    /// 
    /// # Return
    /// 
    /// The new access token
    ///
    pub async fn refresh_token(
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

        let token = Self::get_token(
            self.client_id.clone(),
            self.secret_key.clone(),
            Self::oauth_token_url()?,
            map
        ).await?;

        #[allow(clippy::unwrap_used)]
        {
            *self.access_token.lock().unwrap() = Some(token.access_token.clone());
        }

        Ok(token.access_token)
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
    pub async fn fetch<T>(
        &self,
        path:  &str,
    ) -> Result<T, EveApiError>
    where
        T: DeserializeOwned,
    {
        let path = format!("{}/{}", Self::api_url()?, path);
        let response = self
            .send(&path, &[])
            .await?;

        let data = response
            .json::<T>()
            .await
            .map_err(|x| EveApiError::ReqwestError(x, path))?;
        Ok(data)
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
    pub async fn fetch_auth<T>(
        &self,
        path:  &str,
    ) -> Result<T, EveApiError>
    where
        T: DeserializeOwned,
    {
        let path = format!("{}/{}", Self::api_url()?, path);
        let response = self
            .send_auth(&path, &[])
            .await?;

        let data = response
            .json::<T>()
            .await
            .map_err(|x| EveApiError::ReqwestError(x, path))?;
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
    pub async fn fetch_page<T>(
        &self,
        path:  &str,
    ) -> Result<Vec<T>, EveApiError>
    where
        T: DeserializeOwned + Send,
    {
        let path = format!("{}/{}", Self::api_url()?, path);
        let response = self
            .send(&path, &[])
            .await?;

        let pages = self.page_count(&response);

        if !self.has_content(&response) {
            return Ok(Vec::new());
        }

        let mut data: Vec<T> = Vec::new();
        let fetched_data = response
            .json::<Vec<T>>()
            .await
            .map_err(|x| EveApiError::ReqwestError(x, path.clone()))?;
        data.extend(fetched_data);

        for page in 2..=pages {
            let next_page = self
                .send(
                    &format!("{}", &path),
                    &[("page", &page.to_string())],
                )
                .await?;

            if !self.has_content(&next_page) {
                continue;
            }

            let next_page = next_page
                .json::<Vec<T>>()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, path.clone()))?;
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
        path:  &str,
    ) -> Result<Vec<T>, EveApiError>
    where
        T: std::fmt::Debug + DeserializeOwned + Send,
    {
        let path = format!("{}/{}", Self::api_url()?, path);
        let response = self
            .send_auth(&path, &[])
            .await?;

        let pages = self.page_count(&response);

        if !self.has_content(&response) {
            return Ok(Vec::new());
        }

        let mut data: Vec<T> = Vec::new();
        let fetched_data = response
            .json::<Vec<T>>()
            .await
            .map_err(|x| EveApiError::ReqwestError(x, path.clone()))?;
        data.extend(fetched_data);

        for page in 2..=pages {
            let next_page = self
                .send_auth(
                    &format!("{}", &path),
                    &[("page", &page.to_string())],
                )
                .await?;

            if !self.has_content(&next_page) {
                continue;
            }

            let next_page = next_page
                .json::<Vec<T>>()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, path.clone()))?;
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
    pub async fn post<R, T>(
        &self,
        data: R,
        path: &str
    ) -> Result<T, EveApiError>
    where
        R: Debug + Serialize + Send + Sync,
        T: DeserializeOwned,
    {
        let path = format!("{}/{}", Self::api_url()?, path);
        let json = self
            .send_post(data, &path)
            .await?
            .json::<T>()
            .await
            .map_err(|x| EveApiError::ReqwestError(x, path))?;
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
    async fn send(
        &self,
        path:  &str,
        query: &[(&str, &str)],
    ) -> Result<Response, EveApiError> {
        let entry = {
            self
                .cached_route
                .lock()
                .unwrap()
                .get(&format!("{}?{:?}", path, query))
                .cloned()
                .unwrap_or_default()
        };

        if Utc::now().timestamp() <= entry.expires {
            return Err(EveApiError::DataNotExpired(path.into()));
        }

        let mut retry_counter = 0usize;
        let mut last_status   = StatusCode::OK;
        let mut last_text     = String::new();

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", path);
                return Err(EveApiError::TooManyRetries(
                    path.into(),
                    last_status,
                    last_text,
                ));
            }

            let response = self
                .client
                .get(path)
                .query(query)
                .header(
                    "If-None-Match",
                    entry.etag.clone().unwrap_or_default(),
                )
                .send()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, path.into()))?;

            // Extract the expires and etag
            let expires = response
                .headers()
                .get("expires")
                .map(|x|
                    NaiveDateTime::parse_from_str(
                        &x.to_str().unwrap_or_default(),
                        "%a, %d %b %Y %H:%M:%S %Z"
                    )
                    .unwrap()
                    .and_utc()
                    .timestamp()
                )
                .unwrap_or_default();
            let etag = response
                .headers()
                .get("etag")
                .map(|x| x.to_str().unwrap_or_default().into());

            // Save the expires and etag
            {
                self
                    .cached_route
                    .lock()
                    .unwrap()
                    .insert(
                        format!("{}?{:?}", path, query),
                        CachedRouteInfo {
                            expires,
                            etag,
                        }
                    );
            }

            if response.status() == StatusCode::NOT_FOUND {
                return Err(EveApiError::NotFound(path.into()));
            } else if response.status().as_u16() == 420u16 {
                return Err(EveApiError::RateLimit(path.into()));
            } else if response.status() == StatusCode::NOT_MODIFIED {
                return Err(EveApiError::NotModified(path.into()));
            } else if response.status() == StatusCode::FORBIDDEN ||
               response.status() == StatusCode::UNAUTHORIZED {
                return Err(EveApiError::ClientNotAuthenticated);
            }

            if response.status() == StatusCode::SERVICE_UNAVAILABLE {
                return Err(EveApiError::ServiceUnavailable);
            }

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
                        path = path,
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
    async fn send_auth(
        &self,
        path:  &str,
        query: &[(&str, &str)],
    ) -> Result<Response, EveApiError> {
        let entry = {
            self
                .cached_route
                .lock()
                .unwrap()
                .get(&format!("{}?{:?}", path, query))
                .cloned()
                .unwrap_or_default()
        };
        if Utc::now().timestamp() <= entry.expires {
            return Err(EveApiError::DataNotExpired(path.into()));
        }

        let access_token = {
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        };
        let mut access_token = if access_token.is_none() {
            self.refresh_token().await?;
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
                tracing::error!("Too many retries requesting {}.", path);
                return Err(EveApiError::TooManyRetries(
                    path.into(),
                    last_status,
                    last_text,
                ));
            }

            let token = access_token
                .as_ref()
                .expect("We check but somehow the access_token is still None");
            let response = self
                .client
                .get(path)
                .query(query)
                .bearer_auth(token)
                .header(
                    "If-None-Match",
                    entry.etag.clone().unwrap_or_default()
                )
                .send()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, path.into()))?;

            // Extract the expires and etag
            let expires = response
                .headers()
                .get("expires")
                .map(|x|
                    NaiveDateTime::parse_from_str(
                        &x.to_str().unwrap_or_default(),
                        "%a, %d %b %Y %H:%M:%S %Z"
                    )
                    .unwrap()
                    .and_utc()
                    .timestamp()
                )
                .unwrap_or_default();
            let etag = response
                .headers()
                .get("etag")
                .map(|x| x.to_str().unwrap_or_default().into());

            // Save the expires and etag
            {
                self
                    .cached_route
                    .lock()
                    .unwrap()
                    .insert(
                        format!("{}?{:?}", path, query),
                        CachedRouteInfo {
                            expires,
                            etag,
                        }
                    );
            }

            if response.status() == StatusCode::NOT_FOUND {
                return Err(EveApiError::NotFound(path.into()))
            } else if response.status().as_u16() == 420u16 {
                return Err(EveApiError::RateLimit(path.into()));
            } else if response.status() == StatusCode::NOT_MODIFIED {
                return Err(EveApiError::NotModified(path.into()));
            } else if response.status() == StatusCode::FORBIDDEN ||
               response.status() == StatusCode::UNAUTHORIZED {

                last_status = response.status();
                last_text   = response
                    .text()
                    .await
                    .unwrap_or_default();

                retry_counter += 1;
                access_token = Some(self.refresh_token().await?);

                // Wait a second before trying again
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }

            if response.status() == StatusCode::SERVICE_UNAVAILABLE {
                return Err(EveApiError::ServiceUnavailable);
            }

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
                        path = path,
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
        data: R,
        path: &str,
    ) -> Result<Response, EveApiError>
    where
        R: Debug + Serialize + Send + Sync,
    {
        let access_token = {
            #[allow(clippy::unwrap_used)]
            self.access_token.lock().unwrap().clone()
        };
        let mut access_token = if access_token.is_none() {
            self.refresh_token().await?;
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
                tracing::error!("Too many retries requesting {}.", path);
                return Err(EveApiError::TooManyRetries(
                    path.into(),
                    last_status,
                    last_text,
                ));
            }

            let token = access_token
                .as_ref()
                .expect("We check but somehow the access_token is still None");
            let response = self
                .client
                .post(path)
                .json(&data)
                .bearer_auth(token)
                .send()
                .await
                .map_err(|x| EveApiError::ReqwestError(x, path.into()))?;

            if response.status() == StatusCode::FORBIDDEN ||
               response.status() == StatusCode::UNAUTHORIZED {

                last_status = response.status();
                last_text   = response
                    .text()
                    .await
                    .unwrap_or_default();

                retry_counter += 1;
                access_token = Some(self.refresh_token().await?);

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
                        path = path,
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
        client_id:   EveClientId,
        secret_key:  EveSecretKey,
        oauth_token: Url,
        form:        HashMap<&str, &str>,
    ) -> Result<EveJwtToken, EveApiError> {
        let client_id  = (*client_id).clone();
        let secret_key = (*secret_key).clone();

        let response = Client::new()
            .post(oauth_token.clone())
            .basic_auth(client_id, Some(secret_key))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Host", "login.eveonline.com")
            .form(&form)
            .send()
            .await
            .map_err(EveApiError::GetTokenRequestError)?;

        if response.status().is_success() {
            response
                .json()
                .await
                .map_err(|e| EveApiError::ReqwestError(e, oauth_token.into()))
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
            Self::COMPATIBILITY_DATE_HEADER,
            Self::COMPATIBILITY_DATE_VALUE,
        );

        Client::builder()
            .user_agent(Self::user_agent())
            .default_headers(headers)
            .https_only(true)
            .build()
            .map_err(EveApiError::CouldNotConstructClient)
    }

    fn api_url() -> Result<Url> {
        std::env::var(ENV_API_URL)
            .map(|x| Url::parse(&x).map_err(EveApiError::UrlParseError))
            .unwrap_or(Url::parse("https://esi.evetech.net").map_err(EveApiError::UrlParseError))
    }

    fn oauth_authorization_url() -> Result<Url> {
        std::env::var(ENV_OAUTH_AUTHORIZATION_URL)
            .map(|x| Url::parse(&x).map_err(EveApiError::UrlParseError))
            .unwrap_or(Url::parse("https://login.eveonline.com/v2/oauth/authorize").map_err(EveApiError::UrlParseError))
    }

    fn oauth_jwt_keys_url() -> Result<Url> {
        std::env::var(ENV_OAUTH_JWT_KEYS_URL)
            .map(|x| Url::parse(&x).map_err(EveApiError::UrlParseError))
            .unwrap_or(Url::parse("https://login.eveonline.com/oauth/jwks").map_err(EveApiError::UrlParseError))
    }

    fn oauth_token_url() -> Result<Url> {
        std::env::var(ENV_OAUTH_TOKEN_URL)
            .map(|x| Url::parse(&x).map_err(EveApiError::UrlParseError))
            .unwrap_or(Url::parse("https://login.eveonline.com/v2/oauth/token").map_err(EveApiError::UrlParseError))
    }

    fn user_agent() -> String {
        std::env::var(ENV_USER_AGENT).unwrap_or(String::from("StarFoundry"))
    }
}

#[derive(Clone, Debug, Default)]
struct CachedRouteInfo {
    expires: i64,
    etag:    Option<String>,
}

/// Holds the information about an authenticated character
#[derive(Clone, Debug)]
pub struct AuthenticatedClient {
    /// [CharacterId] of the character this authenticated client belongs to
    pub character_id:   CharacterId,
    /// [CorporationId] of the corporation this authenticated client belongs to
    pub corporation_id: CorporationId,

    refresh_token:       String,
}
