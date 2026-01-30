mod by_player_station;
mod by_region;
mod list_character_orders;
mod list_corporation_orders;
mod price;

pub mod error;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/market`
pub fn routes() -> OpenApiRouter<AppState> {
    let by_player_station = OpenApiRouter::new()
        .routes(routes!(by_player_station::api));

    let by_region = OpenApiRouter::new()
        .routes(routes!(by_region::api));

    let list_character = OpenApiRouter::new()
        .routes(routes!(list_character_orders::api));

    let list_corporation = OpenApiRouter::new()
        .routes(routes!(list_corporation_orders::api));

    let prices = OpenApiRouter::new()
        .routes(routes!(price::api));

    OpenApiRouter::new()
        .merge(by_player_station)
        .merge(by_region)
        .merge(list_character)
        .merge(list_corporation)
        .merge(prices)
}
