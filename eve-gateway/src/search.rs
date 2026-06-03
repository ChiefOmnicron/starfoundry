mod error;
mod search;
mod search_structure;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::error::*;
pub use self::search::*;
use crate::state::AppState;

/// Exposes all routes that are under `/structures`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(search::api));
    let fetch_structure = OpenApiRouter::new()
        .routes(routes!(search_structure::api));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(fetch_structure)
}
