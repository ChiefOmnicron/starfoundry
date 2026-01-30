use reqwest::StatusCode;
use thiserror::Error;
use url::Url;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use axum::Json;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
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
    #[error("the client is not authorized, but needs to be")]
    Unauthorized,
    #[error("the client is forbidden from accessing the resource, {0}")]
    Forbidden(Url),

    #[error("url reqwest error for path '{1}', error: '{0:?}'")]
    ReqwestError(reqwest::Error, Url),
    #[error("generic reqwest error, '{0:?}'")]
    GenericReqwestError(reqwest::Error),
    #[error("error while constructing reqwest client, error: '{0:?}'")]
    CouldNotConstructClient(reqwest::Error),
    #[error("error while parsing url. Validate the environment variables, error: '{0}'")]
    UrlParseError(url::ParseError),
    #[error("error while parsing serde, error: '{0:?}'")]
    SerdeParseError(#[from] serde_json::Error),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
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
            Self::Unauthorized => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::UNAUTHORIZED,
                    Json(
                        ErrorResponse {
                            error: "UNAUTHORIZED".into(),
                            description: format!("{}", self),
                        }
                    )
                ).into_response()
            },
            Self::Forbidden(_) => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::FORBIDDEN,
                    Json(
                        ErrorResponse {
                            error: "FORBIDDEN".into(),
                            description: format!("{}", self),
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

/// Use in error types
#[derive(Serialize)]
pub struct ErrorResponse {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}
