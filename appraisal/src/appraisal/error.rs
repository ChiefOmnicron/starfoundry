use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use starfoundry_gateway::ErrorResponse;
use thiserror::Error;

use crate::api_docs::format_json_errors;

pub type Result<T, E = AppraisalError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AppraisalError {
    #[error("invalid appraisal, additional info: {0}")]
    InvalidAppraisal(String),

    #[error("generic sqlx error: {0}")]
    GenericSqlxError(#[from] sqlx::Error),
    #[error("failed to parse market data, {0}")]
    ParseMarketData(serde_json::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    EveGatewayError(#[from] starfoundry_eve_gateway::EveGatewayError),
    #[error(transparent)]
    MarketLibError(#[from] starfoundry_market::MarketError),
}

impl IntoResponse for AppraisalError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidAppraisal(_) => {
                tracing::info!("{}", self.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error:       "BAD_REQUEST".into(),
                        description: self.to_string(),
                    }),
                )
                    .into_response()
            },
            Self::JsonExtractorRejection(x) => {
                format_json_errors(x).into_response()
            },

            Self::GenericSqlxError(_)
            | Self::ParseMarketData(_)
            | Self::EveGatewayError(_)
            | Self::MarketLibError(_) => {
                tracing::error!("{}", self.to_string());
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse {
                        error:       "UNKNOWN".into(),
                        description:
                            "An unknown error occurred, please try again later."
                                .into(),
                    }),
                )
                    .into_response()
            },
        }
        .into_response()
    }
}
