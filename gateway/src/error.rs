use thiserror::Error;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use axum::Json;
use serde::Serialize;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
#[non_exhaustive]
pub enum Error {
    #[error("the env {0} is not set")]
    EnvNotSet(&'static str),
    #[error("generic reqwest error, error: '{0}'")]
    GenericReqwestError(reqwest::Error),

    #[error("error while fetching JWT-Keys, '{0}'")]
    FetchJwtKey(reqwest::Error),
    #[error("invalid EC jwt key, '{0}'")]
    InvalidES256Key(jsonwebtoken::errors::Error),
    #[error("invalid access_token, '{0}'")]
    InvalidAccessToken(jsonwebtoken::errors::Error),
    #[error("no es256 key")]
    NoEs256Key,

    #[error(transparent)]
    StarFoundryGatewayError(#[from] starfoundry_lib_gateway::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::GenericReqwestError(e)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            _ => {
                dbg!(&self);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        ErrorResponse {
                            error: "INTERNAL_SERVER_ERROR".into(),
                            description: "Unknown error".into(),
                        }
                    )
                ).into_response()
            },
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    /// General error name
    pub error: String,
    /// Human description of the error
    pub description: String,
}
