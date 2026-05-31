mod error;
mod job_planner;

pub use self::job_planner::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    let calculation = OpenApiRouter::new()
        .routes(routes!(job_planner::api));

    OpenApiRouter::new()
        .merge(calculation)
}
