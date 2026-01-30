use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_types::{SystemId, TypeId};
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::universe::error::UniverseError;
use crate::eve_client::error::EveApiError;

pub type Result<T, E = IndustryError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum IndustryError {
    #[error("error while fetching blueprint json '{1}', error: '{0}'")]
    FetchBlueprintJson(sqlx::Error, TypeId),
    #[error("error while fetching system index '{1}', error: '{0}'")]
    FetchSystemIndex(sqlx::Error, SystemId),

    #[error("no system found")]
    NoSystem,

    #[error(transparent)]
    UniverseError(#[from] UniverseError),
    #[error("eve api error, error: '{0:?}'")]
    EveApiError(#[from] EveApiError),
}

impl IntoResponse for IndustryError {
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
