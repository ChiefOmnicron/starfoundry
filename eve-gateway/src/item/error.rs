use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_types::TypeId;
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::auth::AuthError;

pub type Result<T, E = ItemError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ItemError {
    #[error("auth error, error: '{0}'")]
    AuthError(#[from] AuthError),

    #[error("error while fetching item '{1}', error: '{0}'")]
    FetchItem(sqlx::Error, TypeId),
}

impl IntoResponse for ItemError {
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
