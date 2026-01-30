use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/corporations`
pub fn routes() -> OpenApiRouter<AppState> {
    let eve_fetch_assets = OpenApiRouter::new()
        .routes(routes!(crate::asset::fetch_corporation_asset_api));

    let eve_fetch_blueprints = OpenApiRouter::new()
        .routes(routes!(crate::asset::fetch_corporation_blueprint_api));

    OpenApiRouter::new()
        .merge(eve_fetch_assets)
        .merge(eve_fetch_blueprints)
}
