mod error;
mod list_jobs;
mod list;
mod permission;
mod service;

//pub mod permission;

use axum::middleware;
use starfoundry_lib_types::starfoundry_uuid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::project::error::Result;
use crate::project::permission::{assert_exists, assert_read};

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    let list_jobs = OpenApiRouter::new()
        .routes(routes!(list_jobs::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    OpenApiRouter::new()
        .merge(list)
        .merge(list_jobs)
}

starfoundry_uuid!(ProjectUuid, "ProjectUuid");

#[cfg(test)]
pub async fn project_test_routes(
    pool: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use tower::ServiceExt;

    let state = AppState {
        pool: pool.clone(),
    };
    let (app, _) = crate::project::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app
        .clone()
        .oneshot(request)
        .await
        .unwrap()
}
