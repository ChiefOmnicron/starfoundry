mod error;
mod register_player_structure;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let register = OpenApiRouter::new()
        .routes(routes!(register_player_structure::api));

    OpenApiRouter::new()
        .merge(register)
}
