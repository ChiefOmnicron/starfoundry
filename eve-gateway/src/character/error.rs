use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::eve_client::error::EveApiError;

pub type Result<T, E = CharacterError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum CharacterError {
    #[error("error while fetching character, error: '{0}'")]
    FetchCharacter(sqlx::Error),
    #[error("error while inserting character information, error: '{0}'")]
    InsertCharacter(sqlx::Error),

    #[error("error performing eve api call, error: '{0}'")]
    EveApiError(#[from] EveApiError),
}

impl IntoResponse for CharacterError {
    fn into_response(self) -> Response {
        match self {
            Self::EveApiError(EveApiError::NotFound(_)) => {
                tracing::error!("{}", self.to_string());
                (
                    StatusCode::NOT_FOUND,
                    Json(
                        ErrorResponse {
                            error: "NOT_FOUND".into(),
                            description: "The requested resource could not be found.".into(),
                        }
                    )
                ).into_response()
            }

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
