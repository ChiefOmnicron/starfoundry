mod fetch;
mod fetch_bulk;

pub mod error;
pub mod services;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/universe`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_system = OpenApiRouter::new()
        .routes(routes!(fetch::api));

    let fetch_system_bulk = OpenApiRouter::new()
        .routes(routes!(fetch_bulk::api));

    OpenApiRouter::new()
        .merge(fetch_system)
        .merge(fetch_system_bulk)
}
