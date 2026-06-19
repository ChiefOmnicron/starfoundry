use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_eve_client::EveApiError;
use thiserror::Error;

use crate::api_docs::ErrorResponse;

pub type Result<T, E = AssetError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum  AssetError {
    #[error("error while listing blueprints, error: '{0}'")]
    ListBlueprints(sqlx::Error),

    #[error("eve api error, error: '{0:?}'")]
    EveApiError(#[from] EveApiError),
    #[error("gateway error, error: '{0:?}'")]
    GatewayError(#[from] starfoundry_lib_gateway::Error),
}

impl IntoResponse for AssetError {
    fn into_response(self) -> Response {
        match self {
            Self::EveApiError(e) => {
                EveApiError::into_response(e)
            },
            Self::GatewayError(e) => {
                starfoundry_lib_gateway::Error::into_response(e)
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
            }
        }
        .into_response()

    }
}
