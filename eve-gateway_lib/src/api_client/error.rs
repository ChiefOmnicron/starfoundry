use reqwest::StatusCode;
use thiserror::Error;
use url::Url;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum EveGatewayClientError {
    #[error("the env {0} is not set")]
    EnvNotSet(&'static str),

    #[error("resource not found, {0}")]
    NotFound(Url),
    #[error("the server is currently not reachable")]
    ServiceUnavailable,
    #[error("the server is currently not reachable")]
    BadGateway,
    #[error("the request to the given URL failed 3 times in a row, '{0}', '{1}', '{2}'")]
    TooManyRetries(Url, StatusCode, String),
    #[error("the requested resource is currently rate limited, '{0}'")]
    RateLimit(Url),
    #[error("the client is not authorized, but needs to be")]
    Unauthorized,
    #[error("the client is forbidden from accessing the resource, {0}")]
    Forbidden(Url),

    #[error("url reqwest error for path '{1}', error: '{0}'")]
    ReqwestError(reqwest::Error, Url),
    #[error("generic reqwest error, '{0}'")]
    GenericReqwestError(reqwest::Error),
    #[error("error while constructing reqwest client, error: '{0}'")]
    CouldNotConstructClient(reqwest::Error),
    #[error("error while parsing url. Validate the environment variables, error: '{0}'")]
    UrlParseError(url::ParseError),
}
