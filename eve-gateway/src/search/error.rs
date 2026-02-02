use axum::Json;
use axum::response::{IntoResponse, Response};
use reqwest::StatusCode;
use starfoundry_lib_gateway::ErrorResponse;
use thiserror::Error;

use crate::eve_client::error::EveApiError;

pub type Result<T, E = SearchError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum SearchError {
    #[error("the search term must be at least 3 character longs")]
    TooShort,

    #[error("eve api error, error: '{0}'")]
    EveApiError(#[from] EveApiError),
    #[error("gateway error, error: '{0:?}'")]
    GatewayError(#[from] starfoundry_lib_gateway::Error),
}

impl IntoResponse for SearchError {
    fn into_response(self) -> Response {
        match self {
            Self::TooShort => {
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        ErrorResponse {
                            error: "BAD_REQUEST".into(),
                            description: self.to_string(),
                        }
                    )
                ).into_response()
            }

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
