use thiserror::Error;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use starfoundry_lib_gateway::ErrorResponse;
use axum::Json;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("{0}")]
    GatewayClientError(#[from] starfoundry_lib_gateway::error::Error),

    #[error("the env {0} is not set")]
    EnvNotSet(&'static str),
    #[error("error while parsing url. Validate the environment variables, error: '{0}'")]
    UrlParseError(url::ParseError),

    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),

    #[error("the given category '{0}' is not valid, it must be one of: 'agent', 'alliance', 'character', 'constellation', 'corporation', 'faction', 'inventory_type', 'region', 'solar_system', 'station', 'structure'")]
    InvalidSearchCategory(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationError(_) => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        ErrorResponse {
                            error: "INVALID_RESPONSE".into(),
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
