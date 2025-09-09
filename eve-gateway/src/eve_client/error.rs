use reqwest::StatusCode;
use starfoundry_lib_types::CharacterId;
use std::fmt;
use url::Url;

pub type Result<T, E = EveApiError> = std::result::Result<T, E>;

/// Holds all possible errors that can occur in this library.
///
/// Besides that it contains helper functions for easier construction of errors.
///
#[derive(Debug)]
#[non_exhaustive]
pub enum EveApiError {
    /// The payload could not be decoded
    #[deprecated]
    OAuthPayloadDecode(base64::DecodeError),
    /// Could not parse the decoded payload
    #[deprecated]
    OAuthParseError(serde_json::Error),

    /// The requested [CharacterId] could not be found in the cache
    #[deprecated]
    NoSuchIdentity(CharacterId),

    /// An ENV was not set, contains which variable is missing
    EnvNotSet(&'static str),

    /// the application is currently rate limited, '{0}'
    RateLimit(Url),
    /// the resource could not be found, '{0}'
    NotFound(Url),
    /// the route is still cached and new data cannot be obtained, '{0}'
    NotModified(Url),
    /// the last received data is still valid, and the server is not ready to give new data, '{0}'
    DataNotExpired(Url),
    /// the eve Server not reachable
    ServiceUnavailable,
    /// the eve Server not reachable
    BadGateway,
    /// the request to the given URL failed 3 times in a row, '{0}', '{1}', '{2}'
    TooManyRetries(Url, StatusCode, String),

    /// error while fetching eve jwt keys, error: '{0}'
    FetchEveJwtToken(reqwest::Error),
    /// no rs256 key
    NoRs256Key,
    /// the fetched rs256 key from eve couldn't be properly parsed
    InvalidRS256Key,
    /// error while parsing eve jwt token, error: '{0}'
    ParseEveJwtAccessToken(jsonwebtoken::errors::Error),
    /// Failed to parse the character id
    OAuthParseCharacterId(std::num::ParseIntError),

    /// generic reqwest error for path '{1}', error: '{0}'
    ReqwestError(reqwest::Error, Url),
    /// error while constructing reqwest client, error: '{0}'
    CouldNotConstructClient(reqwest::Error),
    /// error while parsing url. Validate the environment variables, error: '{0}'
    UrlParseError(url::ParseError),

    /// the client tried to make a request that requires authentication, but this
    /// client is not authenticated
    ClientNotAuthenticated,
    /// error while parsing token response
    GetTokenError,
    /// error while requesting a new access token, error: '{0}'
    GetTokenRequestError(reqwest::Error),
}

impl std::error::Error for EveApiError { }

impl fmt::Display for EveApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
