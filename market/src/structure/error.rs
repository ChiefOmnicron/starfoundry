use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::api_docs::{ErrorResponse, format_json_errors};

pub type Result<T, E = StructureError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum StructureError {
    #[error("sqlx error: '{0}'")]
    GenericSqlxError(sqlx::Error),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

impl IntoResponse for StructureError {
    fn into_response(self) -> Response {
        match self {
            Self::JsonExtractorRejection(x) => {
                format_json_errors(x).into_response()
            },

            Self::GenericSqlxError(_) => {
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
