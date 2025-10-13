use axum::extract::{FromRef, FromRequestParts};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use reqwest::header::HOST;

use crate::auth::{verify, CharacterInfo};
use crate::state::AppState;

#[derive(Debug)]
pub struct ExtractIdentity {
    pub character_info: CharacterInfo,
    pub is_admin:       bool,
}

impl<S> FromRequestParts<S> for ExtractIdentity
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        let host = if let Some(x) = parts.headers.get(HOST) {
            x.to_str().unwrap_or_default()
        } else {
            tracing::error!("{HOST} header not present");
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "UNAUTHORIZED",
                    "description": "Authenticate and try again"
                }))
            ))
        };

        if let Some(authorization) = parts.headers.get(AUTHORIZATION) {
            let authorization = authorization
                .to_str()
                .unwrap_or_default();

            let verify = verify(
                authorization,
                host,
                state.decoding_key,
            );

            let token = match verify {
                Ok(x) => x,
                Err(e) => {
                    tracing::error!("could not verify {AUTHORIZATION} header, error: {e}");
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(json!({
                            "error": "UNAUTHORIZED",
                            "description": "Authenticate and try again"
                        }))
                    ))
                }
            };

            Ok(ExtractIdentity {
                character_info: token.claims.character_info,
                is_admin:       token.claims.is_admin,
            })
        } else {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "error": "UNAUTHORIZED",
                    "description": "Authenticate and try again"
                }))
            ))
        }
    }
}
