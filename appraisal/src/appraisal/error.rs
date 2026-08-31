use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_gateway::ErrorResponse;
use thiserror::Error;

use crate::api_docs::format_json_errors;
use crate::appraisal::AppraisalCode;

pub type Result<T, E = AppraisalError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppraisalError {
    #[error("the requested appraisal could not be found, {0}")]
    AppraisalNotFound(AppraisalCode),
    #[error("invalid appraisal, additional info: {0}")]
    InvalidAppraisal(String),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    EveGatewayError(#[from] starfoundry_lib_eve_gateway::Error),
    #[error(transparent)]
    MarketLibError(#[from] starfoundry_lib_market::Error),
}

impl IntoResponse for AppraisalError {
    fn into_response(self) -> Response {
        match self {
            Self::AppraisalNotFound(_) => {
                tracing::info!("{}", self.to_string());
                (
                    StatusCode::NOT_FOUND,
                    Json(
                        ErrorResponse {
                            error: "NOT_FOUND".into(),
                            description: self.to_string(),
                        }
                    )
                ).into_response()
            },
            Self::InvalidAppraisal(_) => {
                tracing::info!("{}", self.to_string());
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

            Self::JsonExtractorRejection(x) => {
                format_json_errors(x).into_response()
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
