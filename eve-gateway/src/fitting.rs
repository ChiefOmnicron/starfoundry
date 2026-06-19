mod create;
mod delete;

pub mod error;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/fitting`
pub fn routes() -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new()
        .routes(routes!(create::api));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api));

    OpenApiRouter::new()
        .merge(create)
        .merge(delete)
}
