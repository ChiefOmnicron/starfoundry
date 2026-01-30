use axum::Json;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use starfoundry_lib_types::CharacterId;
use thiserror::Error;
use url::Url;

use crate::api_docs::ErrorResponse;

pub type Result<T, E = EveApiError> = std::result::Result<T, E>;

/// Holds all possible errors that can occur in this library.
///
/// Besides that it contains helper functions for easier construction of errors.
///
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum EveApiError {
    /// The payload could not be decoded
    #[deprecated]
    #[error("dummy")]
    OAuthPayloadDecode(base64::DecodeError),
    /// Could not parse the decoded payload
    #[deprecated]
    #[error("dummy")]
    OAuthParseError(serde_json::Error),

    /// The requested [CharacterId] could not be found in the cache
    #[deprecated]
    #[error("dummy")]
    NoSuchIdentity(CharacterId),

    /// An ENV was not set, contains which variable is missing
    #[error("environment not set {0}")]
    EnvNotSet(&'static str),

    #[error("the application is currently rate limited, '{0}'")]
    RateLimit(Url),
    #[error("the resource could not be found, '{0}'")]
    NotFound(Url),
    #[error("the route is still cached and new data cannot be obtained, '{0}'")]
    NotModified(Url),
    #[error("the last received data is still valid, and the server is not ready to give new data, '{0}'")]
    DataNotExpired(Url),
    #[error("the eve Server not reachable")]
    ServiceUnavailable,
    #[error("the eve Server not reachable")]
    BadGateway,
    #[error("the request to the given URL failed 3 times in a row, '{0}', '{1}', '{2}'")]
    TooManyRetries(Url, StatusCode, String),

    #[error("error while fetching eve jwt keys, error: '{0}'")]
    FetchEveJwtToken(reqwest::Error),
    #[error("no rs256 key")]
    NoRs256Key,
    #[error("the fetched rs256 key from eve couldn't be properly parsed")]
    InvalidRS256Key,
    #[error("error while parsing eve jwt token, error: '{0}'")]
    ParseEveJwtAccessToken(jsonwebtoken::errors::Error),
    #[error("Failed to parse the character id")]
    OAuthParseCharacterId(std::num::ParseIntError),

    #[error("generic reqwest error for path '{1}', error: '{0:?}'")]
    ReqwestError(reqwest::Error, Url),
    #[error("error while constructing reqwest client, error: '{0}'")]
    CouldNotConstructClient(reqwest::Error),
    #[error("error while parsing url. Validate the environment variables, error: '{0}'")]
    UrlParseError(url::ParseError),

    #[error("client is not authenticated")]
    ClientNotAuthenticated,
    #[error("error while parsing token response")]
    GetTokenError,
    #[error("error while requesting a new access token, error: '{0}'")]
    GetTokenRequestError(reqwest::Error),

    #[error("no identity found")]
    NoIdentity,
}

impl IntoResponse for EveApiError {
    fn into_response(self) -> Response {
        match self {
            Self::RateLimit(_) => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::IM_A_TEAPOT,
                    Json(
                        ErrorResponse {
                            error: "RATE_LIMIT".into(),
                            description: format!("{}", self),
                        }
                    )
                ).into_response()
            },
            Self::NotFound(_) => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::NOT_FOUND,
                    Json(
                        ErrorResponse {
                            error: "NOT_FOUND".into(),
                            description: format!("{}", self),
                        }
                    )
                ).into_response()
            },
            Self::DataNotExpired(_) |
            Self::NotModified(_) => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::NOT_MODIFIED,
                    Json(
                        ErrorResponse {
                            error: "NOT_MODIFIED".into(),
                            description: format!("{}", self),
                        }
                    )
                ).into_response()
            },
            Self::ServiceUnavailable => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(
                        ErrorResponse {
                            error: "SERVICE_UNAVAILABLE".into(),
                            description: format!("{}", self),
                        }
                    )
                ).into_response()
            },
            Self::BadGateway => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::BAD_GATEWAY,
                    Json(
                        ErrorResponse {
                            error: "BAD_GATEWAY".into(),
                            description: format!("{}", self),
                        }
                    )
                ).into_response()
            },
            Self::TooManyRetries(_, _, _) => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        ErrorResponse {
                            error: "BAD_REQUEST".into(),
                            description: format!("{}", self),
                        }
                    )
                ).into_response()
            },
            Self::NoIdentity => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::UNAUTHORIZED,
                    Json(
                        ErrorResponse {
                            error: "UNAUTHORIZED".into(),
                            description: "UNAUTHORIZED".into(),
                        }
                    )
                ).into_response()
            },
            _ => {
                tracing::error!("{}", self.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        ErrorResponse {
                            error: "UNKNOWN".into(),
                            description: "An unknown error occurred, please try again later.".into(),
                        }
                    )
                ).into_response()
            },
        }
        .into_response()
    }
}
