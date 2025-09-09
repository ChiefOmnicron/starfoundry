mod error;
mod fetch_system;
mod resolve_structure;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/universe`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_system = OpenApiRouter::new()
        .routes(routes!(fetch_system::api));

    let resolve_structure = OpenApiRouter::new()
        .routes(routes!(resolve_structure::api));

    OpenApiRouter::new()
        .merge(fetch_system)
        .merge(resolve_structure)
}
