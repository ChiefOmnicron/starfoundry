use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use jsonwebtoken::DecodingKey;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use starfoundry_lib_eve_gateway::EveGatewayState;

#[derive(Clone)]
pub struct AppState {
    pub pool:             PgPool,

    pub decoding_key:     Arc<DecodingKey>,
}

impl FromRef<AppState> for EveGatewayState {
    fn from_ref(input: &AppState) -> Self {
        EveGatewayState {
            decoding_key: input.decoding_key.clone(),
        }
    }
}

impl<S> FromRequestParts<S> for AppState
where
    Self: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        _: &mut Parts,
        state: &S
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
