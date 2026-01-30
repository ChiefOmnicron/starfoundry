mod error;
mod fetch;
mod fetch_bulk;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

pub use self::error::*;
pub use self::fetch::*;
pub use self::fetch_bulk::*;

/// returns all routes that are under the path `/characters`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api));

    let fetch_bulk = OpenApiRouter::new()
        .routes(routes!(fetch_bulk::api));

    let eve_fetch_assets = OpenApiRouter::new()
        .routes(routes!(crate::asset::fetch_character_asset_api));

    let eve_fetch_blueprint = OpenApiRouter::new()
        .routes(routes!(crate::asset::fetch_character_blueprint_api));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(fetch_bulk)
        .merge(eve_fetch_assets)
        .merge(eve_fetch_blueprint)
}
