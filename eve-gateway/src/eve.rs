use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes under `/eve`
pub fn routes() -> OpenApiRouter<AppState> {
    let resolve_character_asset = OpenApiRouter::new()
        .routes(routes!(crate::asset::eve_resolve_character_asset::api));
    let resolve_corporation_asset = OpenApiRouter::new()
        .routes(routes!(crate::asset::eve_resolve_corporation_asset::api));

    OpenApiRouter::new()
        // assets
        .merge(resolve_character_asset)
        .merge(resolve_corporation_asset)

        // fittings
        .merge(crate::fitting::routes())
}
