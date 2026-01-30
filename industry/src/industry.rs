mod calculation;
mod error;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let calculation = OpenApiRouter::new()
        .routes(routes!(calculation::api));

    OpenApiRouter::new()
        .merge(calculation)
}
