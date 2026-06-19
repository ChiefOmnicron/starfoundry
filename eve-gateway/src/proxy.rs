mod error;
mod list;
mod list_auth;
mod list_auth_character;
mod list_auth_corporation;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::state::AppState;

/// Exposes all routes that are under `/proxy`
pub fn routes() -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    let list_auth = OpenApiRouter::new()
        .routes(routes!(list_auth::api));

    let list_auth_character = OpenApiRouter::new()
        .routes(routes!(list_auth_character::api));

    let list_auth_corporation = OpenApiRouter::new()
        .routes(routes!(list_auth_corporation::api));

    OpenApiRouter::new()
        .merge(list_auth_character)
        .merge(list_auth_corporation)
        .merge(list_auth)
        .merge(list)
}
