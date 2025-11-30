mod create;
mod delete;
mod error;
mod fetch;
mod list;
mod permission;
mod update;

pub mod service;

use axum::middleware;
use starfoundry_lib_types::starfoundry_uuid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::structure_group::error::Result;
use crate::structure_group::permission::{assert_exists, assert_read, assert_write};

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    let create = OpenApiRouter::new()
        .routes(routes!(create::api));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_write))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_write))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    OpenApiRouter::new()
        .merge(list)
        .merge(fetch)
        .merge(create)
        .merge(update)
        .merge(delete)
}

starfoundry_uuid!(StructureGroupUuid, "StructureGroupUuid");

#[cfg(test)]
pub async fn structure_group_test_routes(
    pool: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use tower::ServiceExt;

    let state: AppState = AppState {
        pool: pool.clone(),
    };
    let (app, _) = crate::structure_group::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app
        .clone()
        .oneshot(request)
        .await
        .unwrap()
}
