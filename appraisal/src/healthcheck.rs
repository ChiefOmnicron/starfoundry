mod healthz;
mod readyz;

use axum::Router;
use axum::routing::get;

pub use self::healthz::*;
pub use self::readyz::*;
use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
}
