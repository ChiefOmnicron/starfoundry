use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_eve_client::EveApiError;
use starfoundry_lib_types::SystemId;
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::auth::error::AuthError;

pub type Result<T, E = SystemError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SystemError {
    #[error("auth error, error: '{0}'")]
    AuthError(#[from] AuthError),

    #[error("error performing eve api call, error: '{0}'")]
    EveApiError(#[from] EveApiError),

    #[error("error while resolving system {1}, error: '{0}'")]
    FetchSystem(sqlx::Error, SystemId),
    #[error("error while resolving systems, error: '{0}'")]
    FetchSystemBulk(sqlx::Error),
    #[error("error while resolving systems, error: '{0}'")]
    ListSystem(sqlx::Error),

    #[error("error while fetching distance for start system {1} and end system {2}, error: '{0}'")]
    FetchSystemDistance(sqlx::Error, SystemId, SystemId),
    #[error("error while listing system distances, error: '{0}'")]
    ListSystemDistance(sqlx::Error),
    #[error("error while listing systems in range for {1}, error: '{0}'")]
    ListSystemsInRange(sqlx::Error, SystemId),
}

impl IntoResponse for SystemError {
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
