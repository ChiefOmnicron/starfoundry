use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::eve_client::error::EveApiError;

pub type Result<T, E = ContractError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ContractError {
    #[error("eve api error, error: '{0}'")]
    EveApiError(#[from] EveApiError),
    #[error("gateway error, error: '{0}'")]
    GatewayError(#[from] starfoundry_lib_gateway::Error),
}

impl IntoResponse for ContractError {
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
