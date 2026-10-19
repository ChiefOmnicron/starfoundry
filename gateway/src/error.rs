use axum::Json;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use serde::Serialize;
use thiserror::Error;
use url::Url;

pub type Result<T, E = GatewayError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum GatewayError {
    #[error("the env {0} is not set")]
    EnvNotSet(&'static str),
    #[error("generic reqwest error, error: '{0:?}'")]
    GenericReqwestError(reqwest::Error),
    #[error("error while constructing reqwest client, error: '{0:?}'")]
    CouldNotConstructClient(reqwest::Error),
    #[error("url reqwest error for path '{1}', error: '{0:?}'")]
    UrlReqwestError(reqwest::Error, Url),

    #[error("error while fetching JWT-Keys, '{0}'")]
    FetchJwtKey(reqwest::Error),
    #[error("invalid EC jwt key, '{0}'")]
    InvalidES256Key(jsonwebtoken::errors::Error),
    #[error("invalid access_token, '{0}'")]
    InvalidAccessToken(jsonwebtoken::errors::Error),
    #[error("no es256 key")]
    NoEs256Key,

    #[error("resource not found, {0}")]
    NotFound(Url),
    #[error("the server is currently not reachable")]
    ServiceUnavailable,
    #[error("the server is currently not reachable")]
    BadGateway,
    #[error(
        "the request to the given URL failed 3 times in a row, '{0}', '{1}', '{2}'"
    )]
    TooManyRetries(Url, StatusCode, String),
    #[error("the client is not authorized, but needs to be")]
    Unauthorized,
    #[error("the client is forbidden from accessing the resource, {0}")]
    Forbidden(Url),

    #[error(
        "error while parsing eve response data, url: '{2}', text: '{1}', error: '{0:?}'"
    )]
    JsonParseError(serde_json::Error, String, Url),
    #[error("error while parsing serde, error: '{0:?}'")]
    SerdeParseError(#[from] serde_json::Error),
}

impl From<reqwest::Error> for GatewayError {
    fn from(e: reqwest::Error) -> Self {
        Self::GenericReqwestError(e)
    }
}

impl IntoResponse for GatewayError {
    fn into_response(self) -> Response {
        #[allow(clippy::match_single_binding)]
        match self {
            _ => {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error:       "INTERNAL_SERVER_ERROR".into(),
                        description: "Unknown error".into(),
                    }),
                )
                    .into_response()
            },
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    /// General error name
    pub error:       String,
    /// Human description of the error
    pub description: String,
}
