mod character;
mod error;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/internal`
pub fn routes() -> OpenApiRouter<AppState> {
    let character = OpenApiRouter::new()
        .routes(routes!(character::api));

    OpenApiRouter::new()
        .merge(character)
}
