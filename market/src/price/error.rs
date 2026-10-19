use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::api_docs::{ErrorResponse, format_json_errors};

pub type Result<T, E = PriceError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum PriceError {
    #[error("error while fetching market price, error: '{0}'")]
    BulkMarketPrice(sqlx::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_eve_gateway::EveGatewayError),
}

impl IntoResponse for PriceError {
    fn into_response(self) -> Response {
        match self {
            Self::JsonExtractorRejection(x) => {
                format_json_errors(x).into_response()
            },

            Self::BulkMarketPrice(_) | Self::EveGatewayLibError(_) => {
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
