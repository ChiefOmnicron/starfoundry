use axum::extract::{FromRef, FromRequestParts, State};
use axum::http::request::Parts;
use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use std::convert::Infallible;

pub type AppStateExtractor = State<AppState>;

#[derive(Clone)]
pub struct AppState {
    pub pool:             PgPool,
    pub credential_cache: Credentials,
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
