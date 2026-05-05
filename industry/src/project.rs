mod add_excess_entry;
mod add_job;
mod add_market;
mod check_resources;
mod create;
mod delete_market_entry;
mod delete;
mod error;
mod fetch_cost;
mod fetch_time_left;
mod fetch;
mod initialize;
mod list_all_jobs;
mod list_excess;
mod list_jobs;
mod list_market_buy;
mod list_market_structures;
mod list_market;
mod list_misc;
mod list;
mod permission;
mod service;
mod split_job_check;
mod update_job;
mod update_market_bulk;
mod update_market_entry;
mod update_misc;
mod update;

use axum::middleware;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::project::permission::{assert_exists, assert_read};

// TODO: check write permission
pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new()
        .routes(routes!(create::api));

    let add_excess = OpenApiRouter::new()
        .routes(routes!(add_excess_entry::api));
    let add_job = OpenApiRouter::new()
        .routes(routes!(add_job::api));
    let add_market = OpenApiRouter::new()
        .routes(routes!(add_market::api));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let fetch_cost = OpenApiRouter::new()
        .routes(routes!(fetch_cost::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let fetch_time_left = OpenApiRouter::new()
        .routes(routes!(fetch_time_left::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let delete_market_entry = OpenApiRouter::new()
        .routes(routes!(delete_market_entry::api));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api));
    let list_jobs = OpenApiRouter::new()
        .routes(routes!(list_jobs::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let list_all_jobs = OpenApiRouter::new()
        .routes(routes!(list_all_jobs::api));
    let list_excess = OpenApiRouter::new()
        .routes(routes!(list_excess::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let list_market = OpenApiRouter::new()
        .routes(routes!(list_market::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let list_market_buy = OpenApiRouter::new()
        .routes(routes!(list_market_buy::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let list_market_structures = OpenApiRouter::new()
        .routes(routes!(list_market_structures::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let list_misc = OpenApiRouter::new()
        .routes(routes!(list_misc::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    let check_resources = OpenApiRouter::new()
        .routes(routes!(check_resources::api));
    let split_job_check = OpenApiRouter::new()
        .routes(routes!(split_job_check::api));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let update_job = OpenApiRouter::new()
        .routes(routes!(update_job::api));
    let update_market_bulk = OpenApiRouter::new()
        .routes(routes!(update_market_bulk::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let update_market_entry = OpenApiRouter::new()
        .routes(routes!(update_market_entry::api));
    let update_misc = OpenApiRouter::new()
        .routes(routes!(update_misc::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));
    let initialize = OpenApiRouter::new()
        .routes(routes!(initialize::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists));

    OpenApiRouter::new()
        .merge(create)
        .merge(add_excess)
        .merge(add_job)
        .merge(add_market)
        .merge(fetch)
        .merge(fetch_cost)
        .merge(fetch_time_left)
        .merge(delete)
        .merge(delete_market_entry)
        .merge(initialize)
        .merge(list)
        .merge(list_excess)
        .merge(list_jobs)
        .merge(list_all_jobs)
        .merge(list_market)
        .merge(list_market_buy)
        .merge(list_market_structures)
        .merge(list_misc)
        .merge(check_resources)
        .merge(split_job_check)
        .merge(update)
        .merge(update_job)
        .merge(update_market_bulk)
        .merge(update_market_entry)
        .merge(update_misc)
}

#[cfg(test)]
pub async fn project_test_routes(
    postgres: sqlx::PgPool,
    request:  axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use tower::ServiceExt;
    use std::sync::Arc;

    use crate::metrics::Metric;

    let state = AppState {
        postgres: postgres.clone(),
        metric:   Arc::new(Metric::new()),
    };
    let (app, _) = crate::project::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app
        .clone()
        .oneshot(request)
        .await
        .unwrap()
}
