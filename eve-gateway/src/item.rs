mod error;
mod fetch;

pub use self::error::*;
pub use self::fetch::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/items`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api));

    OpenApiRouter::new()
        .merge(fetch)
}
