mod list_structure;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let list_structure = OpenApiRouter::new()
        .routes(routes!(list_structure::api));

    OpenApiRouter::new()
        .merge(list_structure)
}
