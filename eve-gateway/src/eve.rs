use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes under `/eve`
pub fn routes() -> OpenApiRouter<AppState> {
    let character_asset = OpenApiRouter::new()
        .routes(routes!(crate::asset::eve_character_asset::api));
    let character_blueprints = OpenApiRouter::new()
        .routes(routes!(crate::asset::eve_character_blueprint::api));
    let corporation_asset = OpenApiRouter::new()
        .routes(routes!(crate::asset::eve_corporation_asset::api));
    let corporation_blueprints = OpenApiRouter::new()
        .routes(routes!(crate::asset::eve_corporation_blueprint::api));

    let system_index = OpenApiRouter::new()
        .routes(routes!(crate::industry::eve_system_index::api));

    let by_player_station = OpenApiRouter::new()
        .routes(routes!(crate::market::eve_by_player_station::api));
    let by_region = OpenApiRouter::new()
        .routes(routes!(crate::market::eve_by_region::api));
    let list_character = OpenApiRouter::new()
        .routes(routes!(crate::market::eve_list_character_orders::api));
    let list_corporation = OpenApiRouter::new()
        .routes(routes!(crate::market::eve_list_corporation_orders::api));
    let prices = OpenApiRouter::new()
        .routes(routes!(crate::market::eve_price::api));

    OpenApiRouter::new()
        // assets
        .merge(character_asset)
        .merge(character_blueprints)
        .merge(corporation_asset)
        .merge(corporation_blueprints)

        // industry
        .merge(system_index)

        // market
        .merge(by_player_station)
        .merge(by_region)
        .merge(list_character)
        .merge(list_corporation)
        .merge(prices)
}
