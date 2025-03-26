use chrono::{NaiveDateTime, Utc};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;
use starfoundry_libs_types::{CorporationId, CharacterId};
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use url::Url;

use crate::{Cache, Error};
use crate::oauth_token::EveOAuthToken;

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
#[derive(Clone, Debug)]
pub struct EveApiClient {
    /// Determines if the client is an authenticated client
    pub authenticated:  Option<AuthenticatedClient>,

    /// Client for communicating with EVE
    client:             Client,

    /// Token needed to get data that is behind auth
    access_token:       Arc<Mutex<Option<String>>>,
    /// Token needed to get data that is behind auth
    cached_route:       Arc<Mutex<HashMap<String, CachedRouteInfo>>>,
}

impl EveApiClient {
    /// URL to the EVE-API
    const EVE_API_URL: &'static str    = "https://esi.evetech.net";
    /// URL to the EVE-API oauth login page
    const EVE_LOGIN_URL: &'static str  = "https://login.eveonline.com/v2/oauth/authorize";
    /// URL to the EVE-API oauth token
    const EVE_TOKEN_URL: &'static str  = "https://login.eveonline.com/v2/oauth/token";
    /// Name of the ENV of the application callback
    const ENV_CALLBACK: &'static str   = "EVE_CALLBACK";
    /// Name of the ENV of the application client id
    const ENV_CLIENT_ID: &'static str  = "EVE_CLIENT_ID";
    /// Name of the ENV of the application secret key
    const ENV_SECRET_KEY: &'static str = "EVE_SECRET_KEY";
    /// Name of the ENV of the user agent
    const ENV_USER_AGENT: &'static str = "EVE_USER_AGENT";

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
    pub async fn access_token(code: &str) -> Result<EveOAuthToken, Error> {
        let mut map = HashMap::new();
        map.insert("grant_type", "authorization_code");
        map.insert("code", code);

        let token = Self::get_token(map).await?;
        Ok(token)
    }

    /// Consutructs a new [EveApiClient].
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
    pub fn new() -> Result<Self, Error> {
        let user_agent = std::env::var(Self::ENV_USER_AGENT)
            .map_err(|_| Error::env_user_agent())?;

        let client = Client::builder()
            .user_agent(user_agent)
            .pool_idle_timeout(None)
            .build()
            .map_err(Error::CouldNotConstructClient)?;

        Ok(Self {
            client:        client,
            access_token:  Arc::new(Mutex::new(None)),
            cached_route:  Arc::new(Mutex::new(HashMap::new())),

            authenticated: None,
        })
    }

    /// Consutructs a new [EveApiClient].
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
        character_id:   CharacterId,
        corporation_id: CorporationId,
        refresh_token:  S,
    ) -> Result<Self, Error>
        where
            S: Into<String> {

        let user_agent = std::env::var(Self::ENV_USER_AGENT)
            .map_err(|_| Error::env_user_agent())?;

        let client = Client::builder()
            .user_agent(user_agent)
            .pool_idle_timeout(None)
            .build()
            .map_err(Error::CouldNotConstructClient)?;

        Ok(Self {
            client:        client,
            access_token:  Arc::new(Mutex::new(None)),
            cached_route:  Arc::new(Mutex::new(HashMap::new())),

            authenticated: Some(AuthenticatedClient {
                refresh_token: refresh_token.into(),
                character_id,
                corporation_id
            }),
        })
    }

    /// Consutructs a new [EveApiClient] with an existing `access_token`.
    ///
    /// # Requirements
    ///
    /// The ENV `EVE_USER_AGENT` must be set.
    ///
    /// # Params
    ///
    /// * `access_token`  -> Access token fromt he EVE-API
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
        character_id:   CharacterId,
        corporation_id: CorporationId,

        access_token:   S,
        refresh_token:  S,
    ) -> Result<Self, Error>
        where
            S: Into<String> {

        let s = Self::new_with_refresh_token(
            character_id,
            corporation_id,
            refresh_token.into(),
        )?;
        #[allow(clippy::unwrap_used)]
        {
            *s.access_token.lock().unwrap() =Some(access_token.into());
        } 
        Ok(s)
    }

    /// Generates a url for authenticationg a character against the EVE-API.
    ///
    /// # Params
    ///
    /// * `state` -> Unique key, used for extra security
    /// * `scope` -> Required scope, musst be a lost of space seperated entries
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
    pub fn auth_uri(state: &str, scope: &str) -> Result<Url, Error> {
        let mut url = Url::parse(Self::EVE_LOGIN_URL).map_err(|_| Error::UrlParseError)?;

        let callback =
            std::env::var(Self::ENV_CALLBACK).map_err(|_| Error::env_callback())?;
        let client_id =
            std::env::var(Self::ENV_CLIENT_ID).map_err(|_| Error::env_client_id())?;
        let _ = std::env::var(Self::ENV_SECRET_KEY).map_err(|_| Error::env_secret_key())?;

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
    pub async fn refresh_token(&self) -> Result<String, Error> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(Error::ClientNotAuthenticated);
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
    /// Returns an error if eiher the request failed or the parsing failed
    ///
    /// # Returns
    ///
    /// Parsed json data
    ///
    pub(crate) async fn fetch<T>(
        &self,
        path:  &str,
        cache: Cache,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let response = self
            .send(&path, &[], cache)
            .await?;

        let data = response
            .json::<T>()
            .await
            .map_err(|x| Error::ReqwestError(x, path))?;
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
    /// Returns an error if eiher the request failed or the parsing failed
    ///
    /// # Returns
    ///
    /// Parsed json data
    ///
    pub(crate) async fn fetch_auth<T>(
        &self,
        path:  &str,
        cache: Cache,
    ) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let response = self
            .send_auth(&path, &[], cache)
            .await?;

        let data = response
            .json::<T>()
            .await
            .map_err(|x| Error::ReqwestError(x, path))?;
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
    /// Returns an error if eiher the request failed or the parsing failed.
    /// The error is returned the first time an error is encountered.
    ///
    /// # Returns
    ///
    /// Vector of parsed json
    ///
    pub(crate) async fn fetch_page<T>(
        &self,
        path:  &str,
        cache: Cache,
    ) -> Result<Vec<T>, Error>
    where
        T: DeserializeOwned + Send,
    {
        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let response = self.send(
            &path,
            &[],
            cache,
        ).await?;

        let pages = self.page_count(&response);

        if !self.has_content(&response) {
            return Ok(Vec::new());
        }

        let mut data: Vec<T> = Vec::new();
        let fetched_data = response
            .json::<Vec<T>>()
            .await
            .map_err(|x| Error::ReqwestError(x, path.clone()))?;
        data.extend(fetched_data);

        for page in 2..=pages {
            let next_page = self
                .send(
                    &format!("{}", &path),
                    &[("page", &page.to_string())],
                    cache,
                )
                .await?;

            if !self.has_content(&next_page) {
                continue;
            }

            let next_page = next_page
                .json::<Vec<T>>()
                .await
                .map_err(|x| Error::ReqwestError(x, path.clone()))?;
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
    /// Returns an error if eiher the request failed or the parsing failed.
    /// The error is returned the first time an error is encountered.
    ///
    /// # Returns
    ///
    /// Vector of parsed json
    ///
    pub(crate) async fn fetch_page_auth<T>(
        &self,
        path:  &str,
        cache: Cache,
    ) -> Result<Vec<T>, Error>
    where
        T: std::fmt::Debug + DeserializeOwned + Send,
    {
        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let response = self.send_auth(
            &path,
            &[],
            cache,
        ).await?;

        let pages = self.page_count(&response);

        if !self.has_content(&response) {
            return Ok(Vec::new());
        }

        let mut data: Vec<T> = Vec::new();
        let fetched_data = response
            .json::<Vec<T>>()
            .await
            .map_err(|x| Error::ReqwestError(x, path.clone()))?;
        data.extend(fetched_data);

        for page in 2..=pages {
            let next_page = self
                .send_auth(
                    &format!("{}", &path),
                    &[("page", &page.to_string())],
                    cache,
                )
                .await?;

            if !self.has_content(&next_page) {
                continue;
            }

            let next_page = next_page
                .json::<Vec<T>>()
                .await
                .map_err(|x| Error::ReqwestError(x, path.clone()))?;
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
    /// Returns an error if eiher the request failed or the parsing failed
    ///
    /// # Returns
    ///
    /// Parsed json data
    ///
    pub(crate) async fn post<R, T>(&self, data: R, path: &str) -> Result<T, Error>
    where
        R: Debug + Serialize + Send + Sync,
        T: DeserializeOwned,
    {
        let path = format!("{}/{}", Self::EVE_API_URL, path);
        let json = self
            .send_post(data, &path)
            .await?
            .json::<T>()
            .await
            .map_err(|x| Error::ReqwestError(x, path))?;
        Ok(json)
    }

    /// Sends a GET request to the given path setting the current `access_token`
    /// as `bearer_auth`.
    ///
    /// If a request failes with a non successfull status, it will retry the
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
    /// retriev a new `access_token`. If that fails an error is returned.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    async fn send(
        &self,
        path:  &str,
        query: &[(&str, &str)],
        cache: Cache,
    ) -> Result<Response, Error> {
        let entry = {
            self
                .cached_route
                .lock()
                .unwrap()
                .get(&format!("{}?{:?}", path, query))
                .cloned()
                .unwrap_or_default()
        };

        if Utc::now().timestamp() <= entry.expires &&
           cache == Cache::Follow {
            return Err(Error::DataNotExpired(path.into()));
        }

        let mut retry_counter = 0usize;
        let mut last_status   = StatusCode::OK;
        let mut last_text     = String::new();

        loop {
            if retry_counter == 3 {
                tracing::error!("Too many retries requesting {}.", path);
                return Err(Error::TooManyRetries(
                    path.into(),
                    last_status,
                    last_text,
                ));
            }

            let mut response = self
                .client
                .get(path)
                .query(query);

            if cache == Cache::Follow {
                response = response.header(
                        "If-None-Match",
                        entry.etag.clone().unwrap_or_default()
                );
            }

            let response = response
                .send()
                .await
                .map_err(|x| Error::ReqwestError(x, path.into()))?;

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
                return Err(Error::NotFound(path.into()));
            } else if response.status().as_u16() == 420u16 {
                return Err(Error::RateLimit(path.into()));
            } else if response.status() == StatusCode::NOT_MODIFIED {
                return Err(Error::NotModified(path.into()));
            } else if response.status() == StatusCode::FORBIDDEN ||
               response.status() == StatusCode::UNAUTHORIZED {
                return Err(Error::ClientNotAuthenticated);
            }

            if response.status() == StatusCode::SERVICE_UNAVAILABLE {
                return Err(Error::ServiceUnavailable);
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
    /// If a request failes with a non successfull status, it will retry the
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
    /// retriev a new `access_token`. If that fails an error is returned.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    async fn send_auth(
        &self,
        path:  &str,
        query: &[(&str, &str)],
        cache: Cache,
    ) -> Result<Response, Error> {
        let entry = {
            self
                .cached_route
                .lock()
                .unwrap()
                .get(&format!("{}?{:?}", path, query))
                .cloned()
                .unwrap_or_default()
        };
        if Utc::now().timestamp() <= entry.expires &&
           cache == Cache::Follow {
            return Err(Error::DataNotExpired(path.into()));
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
                return Err(Error::TooManyRetries(
                    path.into(),
                    last_status,
                    last_text,
                ));
            }

            let token = access_token
                .as_ref()
                .expect("We check but somehow the access_token is still None");
            let mut response = self
                .client
                .get(path)
                .query(query)
                .bearer_auth(token);

            if cache == Cache::Follow {
                response = response.header(
                        "If-None-Match",
                        entry.etag.clone().unwrap_or_default()
                );
            }

            let response = response
                .send()
                .await
                .map_err(|x| Error::ReqwestError(x, path.into()))?;

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
                return Err(Error::NotFound(path.into()))
            } else if response.status().as_u16() == 420u16 {
                return Err(Error::RateLimit(path.into()));
            } else if response.status() == StatusCode::NOT_MODIFIED {
                return Err(Error::NotModified(path.into()));
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
                return Err(Error::ServiceUnavailable);
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
    /// If a request failes with a non successfull status, it will retry the
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
    /// retriev a new `access_token`. If that fails an error is returned.
    ///
    /// # Returns
    ///
    /// Response of the request, ready to work with
    ///
    async fn send_post<R>(
        &self,
        data: R,
        path: &str,
    ) -> Result<Response, Error>
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
                return Err(Error::TooManyRetries(
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
                .map_err(|x| Error::ReqwestError(x, path.into()))?;

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
    /// * `form` -> Form containing `grant_type` and `code` or `refres_token`.
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
        form: HashMap<&str, &str>
    ) -> Result<EveOAuthToken, Error> {
        let client_id =
            std::env::var(Self::ENV_CLIENT_ID).map_err(|_| Error::env_client_id())?;
        let secret_key =
            std::env::var(Self::ENV_SECRET_KEY).map_err(|_| Error::env_secret_key())?;

        let response = Client::new()
            .post(Self::EVE_TOKEN_URL)
            .basic_auth(client_id, Some(secret_key))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("Host", "login.eveonline.com")
            .form(&form)
            .send()
            .await
            .map_err(Error::GetTokenRequestError)?;

        if response.status().is_success() {
            response
                .json()
                .await
                .map_err(Error::GenericReqwestError)
        } else {
            let body = response
                .text()
                .await
                .map_err(Error::GenericReqwestError)?;
            Err(Error::GetTokenError(body))
        }
    }

    /// Extract the page header from the give [reqwest::Response].
    ///
    /// # Params
    ///
    /// * `response` -> Respone to get the header from
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
    /// * `response` -> Respone to get the header from
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
