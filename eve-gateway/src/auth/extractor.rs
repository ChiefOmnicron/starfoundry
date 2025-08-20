use axum::extract::{FromRef, FromRequestParts, Request};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use axum::middleware::Next;
use axum::response::IntoResponse;
use serde_json::json;

use crate::auth::{Identity, JwtToken};
use crate::AppState;

/// Only checks if the authorization header exists
pub async fn assert_login(
    request: Request,
    next: Next,
) -> impl IntoResponse {
    if let None = request
        .headers()
        .get(AUTHORIZATION) {

        return (
            StatusCode::UNAUTHORIZED,
        ).into_response();
    }

    next.run(request).await
}

// TODO: improve to not always hit the database
pub struct ExtractIdentity(pub Identity);

impl<S> FromRequestParts<S> for ExtractIdentity
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        if let Some(authorization) = parts.headers.get(AUTHORIZATION) {
            let token = if let Ok(claim) = JwtToken::verify(authorization.to_str().unwrap()) {
                claim
            } else {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({
                        "error": "UNAUTHORIZED",
                        "description": "Authenticate and try again"
                    }))
                ))
            };

            let state = AppState::from_ref(state);
            Ok(ExtractIdentity(
                Identity::new(
                    state.pool,
                    state.credential_cache,
                    token.claims.character_id(),
                ))
            )
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
