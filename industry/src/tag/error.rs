use axum::Json;
use axum::extract::rejection::JsonRejection;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use starfoundry_gateway::{ErrorResponse, boxed_from};
use thiserror::Error;

use crate::TagUuid;
use crate::api_docs::format_json_errors;

pub type Result<T, E = TagError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
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
    #[error("Validating the input data failed, '{0}'")]
    ValidationError(String),
    #[error("error while parsing string into enum, value: {0}, enum: {1}")]
    EnumParseError(String, &'static str),

    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
    #[error(transparent)]
    IndustryLibError(Box<crate::IndustryError>),
    #[error(transparent)]
    ProjectError(Box<crate::project::error::ProjectError>),
}

impl IntoResponse for TagError {
    fn into_response(self) -> Response {
        match self {
            Self::JsonExtractorRejection(x) => {
                format_json_errors(x).into_response()
            },

            Self::Create(_)
            | Self::Delete(..)
            | Self::Fetch(..)
            | Self::List(_)
            | Self::Update(_)
            | Self::TransactionError(_)
            | Self::SqlxError(_)
            | Self::ValidationError(_)
            | Self::EnumParseError(..)
            | Self::IndustryLibError(_)
            | Self::ProjectError(_) => {
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
    }
}

boxed_from!(TagError::IndustryLibError, crate::IndustryError);
boxed_from!(TagError::ProjectError, crate::project::error::ProjectError);
