use thiserror::Error;
use axum::response::{IntoResponse, Response};
use axum::http::StatusCode;

use crate::api_docs::ErrorResponse;
use axum::Json;

pub type Result<T, E = AuthError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("missing query parameter")]
    InvalidEveLoginResponse,
    #[error("invalid JWT-Token from Eve")]
    InvalidEveJwtToken,
    #[error("invalid Identity")]
    InvalidIdentity,
    #[error("Identity not found in credential cache")]
    IdentityNotFound,

    #[error("error while inserting token, error: '{0}'")]
    InsertTokenError(sqlx::Error),
    #[error("error while getting token, error: '{0}'")]
    GetTokenError(sqlx::Error),

    #[error("error while updating login, error: '{0}'")]
    UpdateLogin(sqlx::Error),

    #[error("error performing eve api call, error: '{0}'")]
    EveApiError(starfoundry_libs_eve_api::Error),

    #[error("missing env 'SECRET_KEY'")]
    MissingEnvSecretKey,
    #[error("error decoding jwt, error '{0}'")]
    JsonWebTokenDecode(jsonwebtoken::errors::Error),
    #[error("error encoding jwt, error '{0}'")]
    JsonWebTokenEncode(jsonwebtoken::errors::Error),

    #[error("generic sqlx error: '{0}'")]
    GenericSqlxError(sqlx::Error),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::IdentityNotFound |
            Self::InvalidEveJwtToken |
            Self::InvalidEveLoginResponse |
            Self::InvalidIdentity => {
                tracing::warn!("{}", self.to_string());
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        ErrorResponse {
                            error: "INVALID_RESPONSE".into(),
                            description: self.to_string(),
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
