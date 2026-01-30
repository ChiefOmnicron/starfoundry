mod public;
mod public_item;

pub mod error;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/contracts`
pub fn routes() -> OpenApiRouter<AppState> {
    let public = OpenApiRouter::new()
        .routes(routes!(public::api));

    let public_item = OpenApiRouter::new()
        .routes(routes!(public_item::api));

    OpenApiRouter::new()
        .merge(public)
        .merge(public_item)
}
