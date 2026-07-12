use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_gateway::ErrorResponse;
use starfoundry_lib_industry::TagUuid;
use thiserror::Error;

use crate::api_docs::format_json_errors;

pub type Result<T, E = TagError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum TagError {
    #[error("error while creating tag, error: '{0}'")]
    Create(sqlx::Error),

    #[error("error while deleting tag '{1}', error: '{0}'")]
    Delete(sqlx::Error, TagUuid),

    #[error("error while fetching tag '{1}', error: '{0}'")]
    Fetch(sqlx::Error, TagUuid),

    #[error("error while listing tags, error: '{0}'")]
    List(sqlx::Error),

    #[error("error while updating tag, error: '{0}'")]
    Update(sqlx::Error),

    #[error("transaction error, '{0}'")]
    TransactionError(sqlx::Error),
    #[error("sqlx error, '{0}'")]
    SqlxError(sqlx::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    ProjectLibError(#[from] starfoundry_lib_industry::Error),
    #[error(transparent)]
    ProjectError(#[from] crate::project::error::ProjectError),
}

impl IntoResponse for TagError {
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
    }
}
