use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_types::StructureId;
use thiserror::Error;

use crate::api_docs::{format_json_errors, ErrorResponse};
use crate::lp::LpError;

pub type Result<T, E = MarketError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MarketError {
    #[error("error while fetching latest market fetch, '{0}'")]
    LatestFetch(sqlx::Error),
    #[error("structure not found, structureId '{0}'")]
    NotFound(StructureId),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    LpError(#[from] LpError),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
}

impl IntoResponse for MarketError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound(_) => {
                (
                    StatusCode::NOT_FOUND,
                    Json(
                        ErrorResponse {
                            error: "NOT_FOUND".into(),
                            description: self.to_string(),
                        }
                    )
                ).into_response()
            }

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
