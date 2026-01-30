mod error;
mod eve_system_index;
mod fetch_blueprint_json;
mod fetch_system_index;
mod service;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/industry`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_system_index = OpenApiRouter::new()
        .routes(routes!(self::fetch_system_index::api));

    let fetch_blueprint_json = OpenApiRouter::new()
        .routes(routes!(self::fetch_blueprint_json::api));

    let eve_system_index = OpenApiRouter::new()
        .routes(routes!(self::eve_system_index::api));

    OpenApiRouter::new()
        .merge(fetch_blueprint_json)
        .merge(fetch_system_index)
        .merge(eve_system_index)
}
