mod create;
mod error;
mod fetch;
mod list_jobs;
mod list_market;
mod list_misc;
mod list;
mod permission;
mod service;

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
    let create = OpenApiRouter::new()
        .routes(routes!(create::api));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    let list_jobs = OpenApiRouter::new()
        .routes(routes!(list_jobs::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    let list_market = OpenApiRouter::new()
        .routes(routes!(list_market::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    let list_misc = OpenApiRouter::new()
        .routes(routes!(list_misc::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    OpenApiRouter::new()
        .merge(create)
        .merge(fetch)
        .merge(list)
        .merge(list_jobs)
        .merge(list_market)
        .merge(list_misc)
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
