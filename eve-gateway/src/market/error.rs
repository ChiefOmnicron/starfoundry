use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::eve_client::error::EveApiError;

pub type Result<T, E = MarketError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum MarketError {
    #[error("eve api error, error: '{0:?}'")]
    EveApiError(#[from] EveApiError),
    #[error("gateway error, error: '{0:?}'")]
    GatewayError(#[from] starfoundry_lib_gateway::Error),
}

impl IntoResponse for MarketError {
    fn into_response(self) -> Response {
        match self {
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
