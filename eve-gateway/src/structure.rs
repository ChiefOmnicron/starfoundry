mod error;
mod fetch_rig;
mod fetch_service;
mod list_structure_rigs;
mod list_structure_services;
mod resolve_structure;

pub mod services;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/structures`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_rig = OpenApiRouter::new()
        .routes(routes!(fetch_rig::api));

    let fetch_services = OpenApiRouter::new()
        .routes(routes!(fetch_service::api));

    let list_structure_rigs = OpenApiRouter::new()
        .routes(routes!(list_structure_rigs::api));

    let list_structure_services = OpenApiRouter::new()
        .routes(routes!(list_structure_services::api));

    let resolve_structure = OpenApiRouter::new()
        .routes(routes!(resolve_structure::api));

    OpenApiRouter::new()
        .merge(fetch_rig)
        .merge(fetch_services)
        .merge(list_structure_rigs)
        .merge(list_structure_services)
        .merge(resolve_structure)
}
