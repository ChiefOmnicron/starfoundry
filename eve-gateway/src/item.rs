mod fetch;
mod fetch_bulk;
mod fetch_bulk_reprocessing;
mod fetch_category;
mod fetch_group;
mod fetch_reprocessing;
mod list;
mod parse;

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

    let fetch_reprocessing = OpenApiRouter::new()
        .routes(routes!(fetch_reprocessing::api));
    let fetch_reprocessing_bulk = OpenApiRouter::new()
        .routes(routes!(fetch_bulk_reprocessing::api));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    let parse = OpenApiRouter::new()
        .routes(routes!(parse::api));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(fetch_bulk)
        .merge(fetch_category)
        .merge(fetch_group)
        .merge(fetch_reprocessing)
        .merge(fetch_reprocessing_bulk)
        .merge(list)
        .merge(parse)
}
