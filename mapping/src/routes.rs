mod error;
mod jump_plan;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/routes`
pub fn routes() -> OpenApiRouter<AppState> {
    let jump_plan = OpenApiRouter::new()
        .routes(routes!(jump_plan::api));

    OpenApiRouter::new()
        .merge(jump_plan)
}
