pub mod eve_resolve_character_asset;
pub mod eve_resolve_corporation_asset;

pub mod list_blueprints;
pub mod service;

mod error;
pub use self::error::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// returns all routes that are under the path `/assets`
pub fn routes() -> OpenApiRouter<AppState> {
    let list_blueprints = OpenApiRouter::new()
        .routes(routes!(self::list_blueprints::api));

    OpenApiRouter::new()
        .merge(list_blueprints)
}
