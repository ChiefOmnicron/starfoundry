mod all;
mod error;
mod service;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::price::error::Result;

pub fn routes() -> OpenApiRouter<AppState> {
    let all = OpenApiRouter::new()
        .routes(routes!(all::api));

    OpenApiRouter::new()
        .merge(all)
}
