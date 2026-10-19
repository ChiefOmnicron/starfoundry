pub mod archive;
pub mod create;
pub mod delete;
pub mod error;
pub mod fetch;
pub mod fetch_members_self;
pub mod list;
pub mod list_default_blacklist;
pub mod list_default_blueprint_overwrites;
pub mod list_default_job_splitting;
pub mod list_default_market;
pub mod list_industry_hub;
pub mod list_members;
pub mod update;
pub mod update_default_blacklist;
pub mod update_default_blueprint_overwrite;
pub mod update_default_job_splitting;
pub mod update_default_market;
pub mod update_industry_hubs;
pub mod update_member;

pub mod permission;

use axum::middleware;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub use self::error::*;
use crate::AppState;
use crate::project_group::permission::{
    assert_exists, assert_owner, assert_read, assert_write
};

pub fn routes(state: AppState) -> OpenApiRouter<AppState> {
    let archive = OpenApiRouter::new()
        .routes(routes!(archive::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_owner,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let create = OpenApiRouter::new().routes(routes!(create::api));

    let list = OpenApiRouter::new().routes(routes!(list::api));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_owner,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let list_members = OpenApiRouter::new()
        .routes(routes!(list_members::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let fetch_members_self = OpenApiRouter::new()
        .routes(routes!(fetch_members_self::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let list_default_blacklist = OpenApiRouter::new()
        .routes(routes!(list_default_blacklist::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let list_default_blueprint_overwrites = OpenApiRouter::new()
        .routes(routes!(list_default_blueprint_overwrites::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let list_default_job_splitting = OpenApiRouter::new()
        .routes(routes!(list_default_job_splitting::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let list_default_market = OpenApiRouter::new()
        .routes(routes!(list_default_market::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let list_industry_hubs = OpenApiRouter::new()
        .routes(routes!(list_industry_hub::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let update_default_blacklist = OpenApiRouter::new()
        .routes(routes!(update_default_blacklist::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let update_default_blueprint_overwrite = OpenApiRouter::new()
        .routes(routes!(update_default_blueprint_overwrite::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let update_default_job_splitting = OpenApiRouter::new()
        .routes(routes!(update_default_job_splitting::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let update_default_market = OpenApiRouter::new()
        .routes(routes!(update_default_market::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let update_industry_hubs = OpenApiRouter::new()
        .routes(routes!(update_industry_hubs::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    let update_members = OpenApiRouter::new()
        .routes(routes!(update_member::api))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_write,
        ))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            assert_exists,
        ));

    OpenApiRouter::new()
        .merge(archive)
        .merge(create)
        .merge(list)
        .merge(update)
        .merge(delete)
        .merge(fetch)
        .merge(list_members)
        .merge(fetch_members_self)
        .merge(list_default_blacklist)
        .merge(list_default_blueprint_overwrites)
        .merge(list_default_job_splitting)
        .merge(list_default_market)
        .merge(list_industry_hubs)
        .merge(update_default_blacklist)
        .merge(update_default_blueprint_overwrite)
        .merge(update_default_job_splitting)
        .merge(update_default_market)
        .merge(update_industry_hubs)
        .merge(update_members)
}

#[cfg(test)]
pub async fn project_group_test_routes(
    postgres: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use std::sync::Arc;

    use tower::ServiceExt;

    use crate::metrics::Metric;

    let state = AppState {
        postgres: postgres.clone(),
        metric:   Arc::new(Metric::new()),
    };
    let (app, _) =
        crate::project_group::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app.clone()
        .oneshot(request)
        .await
        .unwrap()
}
