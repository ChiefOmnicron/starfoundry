use thiserror::Error;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use starfoundry_lib_gateway::{ErrorResponse, boxed_from};
use axum::Json;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    GatewayClientError(Box<starfoundry_lib_gateway::error::Error>),

    #[error("the env {0} is not set")]
    EnvNotSet(&'static str),
    #[error("error while parsing url. Validate the environment variables, error: '{0}'")]
    UrlParseError(url::ParseError),
    #[error("error while parsing string into enum, value: {0}, enum: {1}")]
    EnumParseError(String, &'static str),

    #[error("invalid appraisal, additional info: {0}")]
    InvalidAppraisal(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidAppraisal(_) => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        ErrorResponse {
                            error: "BAD_REQUEST".into(),
                            description: self.to_string(),
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
    }
}

boxed_from!(Error::GatewayClientError, starfoundry_lib_gateway::error::Error);
