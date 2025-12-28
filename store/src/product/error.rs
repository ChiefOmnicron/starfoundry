use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::api_docs::{format_json_errors, ErrorResponse};
use starfoundry_lib_types::CharacterId;

pub type Result<T, E = ProductError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProductError {
    #[error("the character '{0}' is not allowed to access the resource")]
    Forbidden(CharacterId),

    #[error("validation error '{0}'")]
    ValidationError(String),

    #[error("general sqlx error, '{0}'")]
    GeneralSqlxError(sqlx::Error),
    #[error("general serde error, '{0}'")]
    GeneralSerdeError(serde_json::Error),
    #[error("general reqwest error, '{0}'")]
    GeneralReqwestError(reqwest::Error),
    #[error("general GatewayError, error: '{0}'")]
    GatewayError(starfoundry_lib_gateway::error::Error),
    #[error("general EveGatewayError, error: '{0}'")]
    EveGatewayError(starfoundry_lib_eve_gateway::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl From<starfoundry_lib_gateway::error::Error> for ProductError {
    fn from(e: starfoundry_lib_gateway::error::Error) -> Self {
        Self::GatewayError(e)
    }
}

impl From<starfoundry_lib_eve_gateway::Error> for ProductError {
    fn from(e: starfoundry_lib_eve_gateway::Error) -> Self {
        Self::EveGatewayError(e)
    }
}

impl IntoResponse for ProductError {
    fn into_response(self) -> Response {
        match self {
            Self::ValidationError(ref x) => {
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        ErrorResponse {
                            error: "BAD_REQUEST".into(),
                            description: x.into(),
                        }
                    )
                ).into_response()
            }

            Self::GatewayError(_) => {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(
                        ErrorResponse {
                            error: "UNAUTHORIZED".into(),
                            description: "Login and try again".into(),
                        }
                    )
                ).into_response()
            },

            Self::Forbidden(_) => {
                (
                    StatusCode::FORBIDDEN,
                    Json(
                        ErrorResponse {
                            error: "FORBIDDEN".into(),
                            description: "You are not allowed this resource".into(),
                        }
                    )
                ).into_response()
            },

            Self::JsonExtractorRejection(x) => {
                format_json_errors(x).into_response()
            },

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
