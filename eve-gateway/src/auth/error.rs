use axum::http::StatusCode;
use axum::Json;
use axum::response::{IntoResponse, Response};
use starfoundry_lib_eve_client::EveApiError;
use thiserror::Error;

use crate::api_docs::ErrorResponse;
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
    #[error("error while inserting login history, error: '{0}'")]
    InsertLoginHistory(sqlx::Error),

    #[error("error while updating login, error: '{0}'")]
    UpdateLogin(sqlx::Error),

    #[error("character error, error: '{0}'")]
    CharacterError(Box<CharacterError>),
    #[error("error performing eve api call, error: '{0}'")]
    EveApiError(Box<EveApiError>),

    #[error("error decoding jwt, error '{0}'")]
    JsonWebTokenDecode(jsonwebtoken::errors::Error),
    #[error("error encoding jwt, error '{0}'")]
    JsonWebTokenEncode(jsonwebtoken::errors::Error),

    #[error("could not extract coordinates, '{0}'")]
    EcPublicKeyExtractXY(jwk_kit::error::JwkError),
    #[error("could build jwks response, '{0}'")]
    EcPublicKeyBuildResponse(jwk_kit::error::JwkError),
    #[error("error loading ec pem")]
    LoadEcPem(jsonwebtoken::errors::Error),
    #[error("invalid es256 key, '{0}'")]
    InvalidES256Key(jsonwebtoken::errors::Error),
    #[error("invalid access token, '{0}'")]
    InvalidAccessToken(jsonwebtoken::errors::Error),

    #[error("environment not set, '{0}'")]
    EnvNotSet(String),
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        tracing::warn!("{}", self.to_string());

        match self {
            Self::InvalidEveLoginResponse |
            Self::GetAccessTokenError(_) |
            Self::GetRefreshTokenError(_) => {
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

            //Self::EveApiError(EveApiError::BadGateway) => {
            Self::EveApiError(x) => {
                match *x {
                    EveApiError::BadGateway => (
                        StatusCode::BAD_REQUEST,
                        Json(
                            ErrorResponse {
                                error: "BAD_GATEWAY".into(),
                                description: "The EVE-API is currently not reachable, or has some other problems.".into(),
                            }
                        )
                    ).into_response(),
                    _ => (
                        StatusCode::BAD_REQUEST,
                        Json(
                            ErrorResponse {
                                error: "BAD_GATEWAY".into(),
                                description: "The EVE-API is currently not reachable, or has some other problems.".into(),
                            }
                        )
                    ).into_response()
                }
            }

            Self::JsonWebTokenDecode(_) |
            Self::JsonWebTokenEncode(_) => {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(
                        ErrorResponse {
                            error: "UNAUTHORIZED".into(),
                            description: "login and try again.".into(),
                        }
                    )
                ).into_response()
            }

            _ => {
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

// Remove once this_error implements it
// https://github.com/dtolnay/thiserror/issues/424
// https://github.com/dtolnay/thiserror/pull/431
#[macro_export]
macro_rules! boxed_from {
    ($dst_ty:ident :: $variant:ident, $src_ty:ty) => {
        impl From<$src_ty> for $dst_ty {
            fn from(value: $src_ty) -> Self {
                Self::$variant(Box::new(value))
            }
        }
    };
}

boxed_from!(AuthError::CharacterError, CharacterError);
boxed_from!(AuthError::EveApiError, EveApiError);
