mod create;
mod delete;
mod error;
mod list;

use utoipa_axum::routes;
use utoipa_axum::router::OpenApiRouter;
use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new()
        .routes(routes!(create::api));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    OpenApiRouter::new()
        .merge(create)
        .merge(delete)
        .merge(list)
}
