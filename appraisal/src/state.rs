use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use sqlx::PgPool;
use std::convert::Infallible;
use std::sync::Arc;

use crate::metrics::Metric;

#[derive(Clone)]
pub struct AppState
where {
    /// Postgres connection pool
    pub postgres: PgPool,
    /// Track metrics for the application
    pub metric:   Arc<Metric>,
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
