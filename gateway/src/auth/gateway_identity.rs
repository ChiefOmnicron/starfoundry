use axum::Json;
use axum::extract::{FromRef, FromRequestParts, OptionalFromRequestParts};
use axum::http::StatusCode;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use reqwest::header::HOST;
use serde_json::json;

use crate::auth::{CharacterInfo, verify};
use crate::state::AppState;

#[derive(Debug)]
pub struct GatewayExtractIdentity {
    pub character_info: CharacterInfo,
    pub is_admin:       bool,
}

impl<S> FromRequestParts<S> for GatewayExtractIdentity
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let host = if let Some(x) = parts
            .headers
            .get(HOST)
        {
            x.to_str()
                .unwrap_or_default()
        } else {
            tracing::error!("{HOST} header not present");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "UNAUTHORIZED",
                    "description": "Authenticate and try again"
                })),
            ))
        };

        if let Some(authorization) = parts
            .headers
            .get(AUTHORIZATION)
        {
            let authorization = authorization
                .to_str()
                .unwrap_or_default();

            let verify = verify(authorization, host, state.decoding_key);

            let token = match verify {
                Ok(x) => x,
                Err(e) => {
                    tracing::error!(
                        "could not verify {AUTHORIZATION} header, error: {e}"
                    );
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "error": "UNAUTHORIZED",
                            "description": "Authenticate and try again"
                        })),
                    ))
                },
            };

            Ok(GatewayExtractIdentity {
                character_info: token
                    .claims
                    .character_info,
                is_admin:       token
                    .claims
                    .is_admin,
            })
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "UNAUTHORIZED",
                    "description": "Authenticate and try again"
                })),
            ))
        }
    }
}

impl<S> OptionalFromRequestParts<S> for GatewayExtractIdentity
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<Self>, Self::Rejection> {
        let state = AppState::from_ref(state);

        let host = if let Some(x) = parts
            .headers
            .get(HOST)
        {
            x.to_str()
                .unwrap_or_default()
        } else {
            tracing::error!("{HOST} header not present");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "UNAUTHORIZED",
                    "description": "Authenticate and try again"
                })),
            ))
        };

        if let Some(authorization) = parts
            .headers
            .get(AUTHORIZATION)
        {
            let authorization = authorization
                .to_str()
                .unwrap_or_default();

            let verify = verify(authorization, host, state.decoding_key);

            let token = match verify {
                Ok(x) => x,
                Err(e) => {
                    tracing::error!(
                        "could not verify {AUTHORIZATION} header, error: {e}"
                    );
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "error": "UNAUTHORIZED",
                            "description": "Authenticate and try again"
                        })),
                    ))
                },
            };

            Ok(Some(GatewayExtractIdentity {
                character_info: token
                    .claims
                    .character_info,
                is_admin:       token
                    .claims
                    .is_admin,
            }))
        } else {
            Ok(None)
        }
    }
}
