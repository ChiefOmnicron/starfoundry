mod healthz;
mod readyz;

use axum::routing::get;
use axum::Router;

use crate::state::AppState;

pub use self::healthz::*;
pub use self::readyz::*;

/// Returns all routes under the `/healthcheck` path
pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
}
