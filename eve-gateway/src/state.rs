use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;
use std::collections::HashMap;

use crate::config::ConfigFileDomain;

/// State that can be used in every route
#[derive(Clone)]
pub struct AppState {
    /// Postgres connection pool
    pub postgres:       PgPool,
    /// Valid domains read from the config file
    pub auth_domains:   Arc<HashMap<String, ConfigFileDomain>>,
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
