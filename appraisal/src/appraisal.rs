mod compression;
mod create;
mod error;
mod fetch;
mod reprocessing;
mod settings;

pub use self::error::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new()
        .routes(routes!(create::api));
    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api));

    //let compression = OpenApiRouter::new()
    //    .routes(routes!(compression::api));
    let reprocessing = OpenApiRouter::new()
        .routes(routes!(reprocessing::api));

    //let settings = OpenApiRouter::new()
    //    .routes(routes!(settings::api));

    OpenApiRouter::new()
        .merge(create)
        .merge(fetch)

        //.merge(compression)
        .merge(reprocessing)

        //.merge(settings)
}
