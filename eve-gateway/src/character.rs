mod error;
mod fetch;
mod fetch_bulk;

pub mod service;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

pub use self::error::*;

/// returns all routes that are under the path `/characters`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(self::fetch::api));

    let fetch_bulk = OpenApiRouter::new()
        .routes(routes!(self::fetch_bulk::api));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(fetch_bulk)
}
