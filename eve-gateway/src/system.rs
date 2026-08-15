mod fetch;
mod fetch_bulk;
mod fetch_distance;
mod list;
mod list_distances;
mod list_in_range;

pub mod error;
pub mod services;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/systems`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch_system = OpenApiRouter::new()
        .routes(routes!(fetch::api));
    let fetch_system_bulk = OpenApiRouter::new()
        .routes(routes!(fetch_bulk::api));
    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    let fetch_distance = OpenApiRouter::new()
        .routes(routes!(fetch_distance::api));
    let list_distances = OpenApiRouter::new()
        .routes(routes!(list_distances::api));
    let list_in_range = OpenApiRouter::new()
        .routes(routes!(list_in_range::api));

    OpenApiRouter::new()
        .merge(fetch_system)
        .merge(fetch_system_bulk)
        .merge(list)

        .merge(fetch_distance)
        .merge(list_distances)
        .merge(list_in_range)
}
