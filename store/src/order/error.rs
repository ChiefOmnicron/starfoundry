use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::api_docs::{format_json_errors, ErrorResponse};

pub type Result<T, E = OrderError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum OrderError {
    #[error("general EveGatewayError, error: '{0}'")]
    EveGatewayError(starfoundry_lib_eve_gateway::Error),
    #[error("general ProductError, error: '{0}'")]
    ProductError(crate::product::ProductError),

    #[error("general sqlx error, '{0}'")]
    GeneralSqlxError(sqlx::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl From<starfoundry_lib_eve_gateway::Error> for OrderError {
    fn from(e: starfoundry_lib_eve_gateway::Error) -> Self {
        Self::EveGatewayError(e)
    }
}

impl From<crate::product::ProductError> for OrderError {
    fn from(e: crate::product::ProductError) -> Self {
        Self::ProductError(e)
    }
}

impl IntoResponse for OrderError {
    fn into_response(self) -> Response {
        match self {
            Self::EveGatewayError(starfoundry_lib_eve_gateway::Error::AuthError(_)) => {
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
