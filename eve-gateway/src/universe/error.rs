use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_types::SystemId;
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::eve_client::error::EveApiError;
use crate::auth::AuthError;
use crate::item::ItemError;

pub type Result<T, E = UniverseError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum UniverseError {
    #[error("auth error, error: '{0}'")]
    AuthError(#[from] AuthError),
    #[error("item error, error: '{0}'")]
    ItemError(#[from] ItemError),

    #[error("error performing eve api call, error: '{0}'")]
    EveApiError(#[from] EveApiError),

    #[error("the item information are not available")]
    ItemNotFound,

    #[error("error while resolving system {1}, error: '{0}'")]
    FetchSystem(sqlx::Error, SystemId),
    #[error("error while resolving systems, error: '{0}'")]
    FetchSystemBulk(sqlx::Error),
}

impl IntoResponse for UniverseError {
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
