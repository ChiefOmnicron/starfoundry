use axum::Json;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use starfoundry_lib_eve_client::EveApiError;
use starfoundry_lib_gateway::ErrorResponse;
use thiserror::Error;

pub type Result<T, E = ProxyError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProxyError {
    #[error("No scope found the requested url")]
    NoScopeFound,

    #[error("eve api error, error: '{0:?}'")]
    EveApiError(#[from] EveApiError),
    #[error("gateway error, error: '{0:?}'")]
    GatewayError(#[from] starfoundry_lib_gateway::Error),
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        match self {
            Self::NoScopeFound => {
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        ErrorResponse {
                            error: "BAD_REQUEST".into(),
                            description: self.to_string(),
                        }
                    )
                ).into_response()
            },
            Self::EveApiError(e) => {
                EveApiError::into_response(e)
            },
            Self::GatewayError(e) => {
                starfoundry_lib_gateway::Error::into_response(e)
            },
        }
        .into_response()
    }
}
