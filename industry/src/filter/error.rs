use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use starfoundry_gateway::ErrorResponse;
use thiserror::Error;

use crate::FilterUuid;
use crate::api_docs::format_json_errors;

pub type Result<T, E = FilterError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum FilterError {
    #[error("filter with id '{0}' not found")]
    NotFound(FilterUuid),

    #[error("error while listing filter, error: '{0}'")]
    List(sqlx::Error),

    #[error("error while creating filter, error: '{0}'")]
    Create(sqlx::Error),

    #[error("error while deleting filter, error: '{0}'")]
    Delete(sqlx::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for FilterError {
    fn into_response(self) -> Response {
        match self {
            Self::NotFound(_) => {
                tracing::info!("{}", self.to_string());
                (
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse {
                        error:       "NOT_FOUND".into(),
                        description: self.to_string(),
                    }),
                )
                    .into_response()
            },

            Self::JsonExtractorRejection(x) => {
                format_json_errors(x).into_response()
            },

            Self::List(_) | Self::Create(_) | Self::Delete(_) => {
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
