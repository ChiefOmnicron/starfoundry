use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use std::collections::HashMap;

use crate::client::ConfigEveApi;
use crate::config::ConfigFileDomain;

#[derive(Clone)]
pub struct AppState {
    pub postgres:       PgPool,
    pub auth_domains:   Arc<HashMap<String, ConfigFileDomain>>,
    pub eve_api:        Arc<ConfigEveApi>,
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
