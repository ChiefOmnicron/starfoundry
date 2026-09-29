mod error;
mod list;

use utoipa_axum::routes;
use utoipa_axum::router::OpenApiRouter;
use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    OpenApiRouter::new()
        .merge(list)
}
