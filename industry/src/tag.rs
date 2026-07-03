mod create;
mod delete;
mod fetch;
mod list;
mod update;

pub mod error;
pub mod service;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api));

    let create = OpenApiRouter::new()
        .routes(routes!(create::api));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api));

    OpenApiRouter::new()
        .merge(list)
        .merge(fetch)
        .merge(create)
        .merge(update)
        .merge(delete)
}
