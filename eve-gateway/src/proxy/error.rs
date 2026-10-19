use axum::Json;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use starfoundry_gateway::{ErrorResponse, boxed_from};
use starfoundry_lib_eve_client::EveApiError;
use thiserror::Error;
use url::Url;

pub type Result<T, E = ProxyError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ProxyError {
    #[error("no scope found the requested url")]
    NoScopeFound,

    #[error("generic reqwest error for path '{1}', error: '{0:?}'")]
    ReqwestError(reqwest::Error, Url),

    #[error("eve api error, error: '{0:?}'")]
    EveApiError(Box<EveApiError>),
    #[error("gateway error, error: '{0:?}'")]
    GatewayError(Box<starfoundry_gateway::GatewayError>),
}

impl IntoResponse for ProxyError {
    fn into_response(self) -> Response {
        match self {
            Self::NoScopeFound | Self::ReqwestError(..) => {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse {
                        error:       "BAD_REQUEST".into(),
                        description: self.to_string(),
                    }),
                )
                    .into_response()
            },
            Self::EveApiError(e) => EveApiError::into_response(*e),
            Self::GatewayError(e) => {
                starfoundry_gateway::GatewayError::into_response(*e)
            },
        }
        .into_response()
    }
}

boxed_from!(ProxyError::EveApiError, EveApiError);
boxed_from!(ProxyError::GatewayError, starfoundry_gateway::GatewayError);
