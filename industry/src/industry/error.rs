use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::api_docs::{format_json_errors, ErrorResponse};

pub type Result<T, E = IndustryError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IndustryError {
    #[error("error while parsing dependency, {0}")]
    ParseJsonToDependency(serde_json::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    EveGatewayLibError(#[from] starfoundry_lib_eve_gateway::Error),
    #[error(transparent)]
    MarketLibError(#[from] starfoundry_lib_market::Error),
    #[error(transparent)]
    ProjectGroupError(#[from] crate::project_group::ProjectGroupError),
}

impl IntoResponse for IndustryError {
    fn into_response(self) -> Response {
        match self {
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
