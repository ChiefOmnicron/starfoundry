mod error;
mod list_structure_rigs;
mod fetch_rig;
mod fetch_services;
mod resolve_structure;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/structures`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_rig_information = OpenApiRouter::new()
        .routes(routes!(fetch_rig::api));

    let fetch_structure_rigs = OpenApiRouter::new()
        .routes(routes!(list_structure_rigs::api));

    let fetch_services = OpenApiRouter::new()
        .routes(routes!(fetch_services::api));

    let resolve_structure = OpenApiRouter::new()
        .routes(routes!(resolve_structure::api));

    OpenApiRouter::new()
        .merge(fetch_rig_information)
        .merge(fetch_structure_rigs)
        .merge(fetch_services)
        .merge(resolve_structure)
}
