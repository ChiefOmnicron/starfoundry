mod healthz;
mod readyz;

use axum::routing::get;
use axum::Router;

use crate::AppState;

pub use self::healthz::*;
pub use self::readyz::*;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
}
