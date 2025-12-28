mod error;
mod search;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::error::*;
pub use self::search::*;
use crate::state::AppState;

/// Exposes all routes that are under `/structures`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(search::api));

    OpenApiRouter::new()
        .merge(fetch)
}
