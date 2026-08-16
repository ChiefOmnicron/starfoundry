mod error;
mod bulk;
mod last_fetch;
mod service;
mod virtual_market;

pub use self::service::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let bulk = OpenApiRouter::new()
        .routes(routes!(bulk::api));

    let last_fetch = OpenApiRouter::new()
        .routes(routes!(last_fetch::api));

    let virtual_market = OpenApiRouter::new()
        .routes(routes!(virtual_market::api));

    OpenApiRouter::new()
        .merge(bulk)
        .merge(last_fetch)
        .merge(virtual_market)
}
