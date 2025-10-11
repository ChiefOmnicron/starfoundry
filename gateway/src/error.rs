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
