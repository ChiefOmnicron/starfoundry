use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::api_docs::ErrorResponse;
use crate::eve_client::error::EveApiError;
use crate::character::CharacterError;

pub type Result<T, E = AuthError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum AuthError {
    #[error("missing query parameter")]
    InvalidEveLoginResponse,

    #[error("error while getting access_token, error: '{0}'")]
    GetAccessTokenError(sqlx::Error),
    #[error("error while getting refresh_token, error: '{0}'")]
    GetRefreshTokenError(sqlx::Error),
    #[error("error while inserting eve refresh_token, error: '{0}'")]
    InsertRefreshToken(sqlx::Error),
    #[error("error while inserting credentials, error: '{0}'")]
    InsertEveCredential(sqlx::Error),
    #[error("error while inserting login attempt, error: '{0}'")]
    InsertLoginAttempt(sqlx::Error),

    #[error("error while updating login, error: '{0}'")]
    UpdateLogin(sqlx::Error),

    #[error("character error, error: '{0}'")]
    CharacterError(#[from] CharacterError),
    #[error("error performing eve api call, error: '{0}'")]
    EveApiError(#[from] EveApiError),

    #[error("error decoding jwt, error '{0}'")]
    JsonWebTokenDecode(jsonwebtoken::errors::Error),
    #[error("error encoding jwt, error '{0}'")]
    JsonWebTokenEncode(jsonwebtoken::errors::Error),

    #[error("could not extract coordinates, {0}")]
    EcPublicKeyExtractXY(jwk_kit::error::JwkError),
    #[error("could build jwks response, {0}")]
    EcPublicKeyBuildResponse(jwk_kit::error::JwkError),
    #[error("error loading ec pem")]
    LoadEcPem(jsonwebtoken::errors::Error),

    #[error("environment not set, '{0}'")]
    EnvNotSet(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidEveLoginResponse |
            Self::GetAccessTokenError(_) |
            Self::GetRefreshTokenError(_) => {
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
