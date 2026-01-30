use axum::extract::{FromRef, FromRequestParts};
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;
use axum::http::StatusCode;
use axum::Json;
use serde_json::json;
use sqlx::PgPool;

use crate::auth::error::{AuthError, Result};
use crate::auth::verify;
use crate::character::CharacterInfo;
use crate::eve_client::EveApiClient;
use crate::state::AppState;

pub struct ExtractIdentity {
    pub character_info: CharacterInfo,
}

impl ExtractIdentity {
    pub async fn eve_api_client(
        &self,
        postgres: &PgPool,
    ) -> Result<Option<EveApiClient>> {
        let refresh_token = sqlx::query!("
                SELECT refresh_token
                FROM eve_credential
                WHERE character_id = $1
            ",
                *self.character_info.character_id,
            )
            .fetch_optional(postgres)
            .await
            .map_err(AuthError::GetRefreshTokenError)?;

        if let Some(x) = refresh_token {
            Ok(
                Some(
                    EveApiClient::new_with_refresh_token(
                        self.character_info.character_id,
                        x.refresh_token,
                    )?
                )
            )
        } else {
            Ok(None)
        }
    }
}

impl<S> FromRequestParts<S> for ExtractIdentity
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let state = AppState::from_ref(state);

        if let Some(authorization) = parts.headers.get(AUTHORIZATION) {
            let authorization = authorization
                .to_str()
                .unwrap_or_default();

            let audience = state
                .auth_domains
                .keys()
                .cloned()
                .collect::<Vec<_>>();
            let verify = verify(
                authorization,
                audience,
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
