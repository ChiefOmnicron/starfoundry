mod fetch;
mod list;
mod update;

use axum::middleware;
use starfoundry_lib_gateway::assert_admin;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn(assert_admin));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api))
        .route_layer(middleware::from_fn(assert_admin));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn(assert_admin));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(list)
        .merge(update)
}
