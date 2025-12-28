mod by_player_station;
mod by_region;

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

    OpenApiRouter::new()
        .merge(by_player_station)
        .merge(by_region)
}
