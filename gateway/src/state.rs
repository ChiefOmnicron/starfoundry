use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use std::convert::Infallible;
use std::sync::Arc;
use std::collections::HashMap;

use crate::config::ConfigFileRoute;

/// State that can be used in every route
#[derive(Clone)]
pub struct AppState {
    /// Valid domains read from the config file
    pub routes: Arc<HashMap<String, ConfigFileRoute>>,
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
