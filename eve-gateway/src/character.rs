mod error;
mod fetch;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub use self::error::*;
pub use self::fetch::*;

pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api));

    OpenApiRouter::new()
        .merge(fetch)
}
