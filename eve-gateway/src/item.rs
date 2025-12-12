mod fetch;
mod fetch_bulk;
mod fetch_category;
mod fetch_group;
mod list;

pub mod services;
pub mod error;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/items`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api));

    let fetch_bulk = OpenApiRouter::new()
        .routes(routes!(fetch_bulk::api));

    let fetch_category = OpenApiRouter::new()
        .routes(routes!(fetch_category::api));

    let fetch_group = OpenApiRouter::new()
        .routes(routes!(fetch_group::api));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(fetch_bulk)
        .merge(fetch_category)
        .merge(fetch_group)
        .merge(list)
}
