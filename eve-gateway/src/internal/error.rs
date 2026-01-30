use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::eve_client::error::EveApiError;

pub type Result<T, E = InternalError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum InternalError {
    #[error("error while fetching characters, error: '{0}'")]
    FetchCharacter(sqlx::Error),

    #[error("error performing eve api call, error: '{0}'")]
    EveApiError(#[from] EveApiError),
}

impl IntoResponse for InternalError {
    fn into_response(self) -> Response {
        match self {
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
