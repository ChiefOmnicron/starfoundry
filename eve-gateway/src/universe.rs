pub mod error;
pub mod fetch_system;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/universe`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_system = OpenApiRouter::new()
        .routes(routes!(fetch_system::api));

    OpenApiRouter::new()
        .merge(fetch_system)
}
