use reqwest::StatusCode;
use starfoundry_libs_types::CharacterId;
use std::fmt;

/// Holds all possible errors that can occur in this library.
///
/// Besides that it contains helper functions for easier construction of errors.
///
#[derive(Debug)]
pub enum Error {
    /// An ENV was not set, contains which variable is missing
    EnvNotSet(String),
    /// The EVE-Client could not be constructed
    CouldNotConstructClient(reqwest::Error),
    /// Could not parse the given url
    UrlParseError,
    /// The request to the given URL failed 3 times in a row
    TooManyRetries(String, StatusCode, String),

    /// Generic reqwest error
    ReqwestError(reqwest::Error, String),
    /// Error that happend while getting a new token
    GetTokenRequestError(reqwest::Error),
    /// Error that happend while getting a new token
    GetTokenError(String),

    /// The payload could not be decoded
    OAuthPayloadDecode(base64::DecodeError),
    /// Could not parse the decoded payload
    OAuthParseError(serde_json::Error),
    /// Failed to parse the character id
    OAuthParseCharacterId(std::num::ParseIntError),
    /// GenericReqwestError
    GenericReqwestError(reqwest::Error),

    /// The application is rate limited
    RateLimit(String),
    /// The ressource couldn't be found
    NotFound(String),
    /// The route is still cached
    NotModified(String),
    /// The last received data is still valid, and the server is not ready to
    /// give new data
    DataNotExpired(String),
    /// The client tried to make an request that requires an authenticated user
    ClientNotAuthenticated,
    /// Eve Server not reachable
    ServiceUnavailable,

    /// The requested [CharacterId] could not be found in the cache
    NoSuchIdentity(CharacterId),
}

impl Error {
    /// Returns an error that the ENV `EVE_USER_AGENT` is not set
    ///
    pub fn env_user_agent() -> Self {
        Self::EnvNotSet("ENV 'EVE_USER_AGENT' is not set!".into())
    }

    /// Returns an error that the ENV `EVE_CALLBACK` is not set
    ///
    pub fn env_callback() -> Self {
        Self::EnvNotSet("ENV 'EVE_CALLBACK' is not set!".into())
    }

    /// Returns an error that the ENV `EVE_CLIENT_ID` is not set
    ///
    pub fn env_client_id() -> Self {
        Self::EnvNotSet("ENV 'EVE_CLIENT_ID' is not set!".into())
    }

    /// Returns an error that the ENV `EVE_SECRET_KEY` is not set
    ///
    pub fn env_secret_key() -> Self {
        Self::EnvNotSet("ENV 'EVE_SECRET_KEY' is not set!".into())
    }
}

impl std::error::Error for Error { }

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
