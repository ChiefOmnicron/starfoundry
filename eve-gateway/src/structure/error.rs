use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_types::TypeId;
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::eve_client::error::EveApiError;
use crate::auth::error::AuthError;
use crate::item::error::ItemError;
use crate::universe::error::UniverseError;

pub type Result<T, E = StructureError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum StructureError {
    #[error("auth error, error: '{0}'")]
    AuthError(#[from] AuthError),
    #[error("item error, error: '{0}'")]
    ItemError(#[from] ItemError),
    #[error("item error, error: '{0}'")]
    UniverseError(#[from] UniverseError),

    #[error("error performing eve api call, error: '{0}'")]
    EveApiError(#[from] EveApiError),

    #[error("error while fetching structure rigs, type_id: '{1}', error: '{0}'")]
    FetchStructureRigs(sqlx::Error, TypeId),
    #[error("error while fetching rig information, type_id: '{1}', error: '{0}'")]
    FetchRigInformation(sqlx::Error, TypeId),
    #[error("error while fetching structure services, type_id: '{1}', error: '{0}'")]
    FetchStructureServices(sqlx::Error, TypeId),

    #[error("the item information are not available")]
    ItemNotFound,
    #[error("the system information are not available")]
    SystemNotFound,
}

impl IntoResponse for StructureError {
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
