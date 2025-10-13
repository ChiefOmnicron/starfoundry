mod create;
mod delete;
mod error;
mod fetch_members_self;
mod fetch;
mod list_default_blacklist;
mod list_default_market;
mod list_members;
mod list;
mod permission;
mod update;

use axum::extract::{Path, Request, State};
use axum::middleware;
use axum::middleware::Next;
use axum::response::IntoResponse;
use starfoundry_lib_types::starfoundry_uuid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::project_group::error::Result;
use starfoundry_lib_eve_gateway::{assert_login, ExtractIdentity};

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let create = OpenApiRouter::new()
        .routes(routes!(create::api))
        .route_layer(middleware::from_fn(assert_login));

    let list = OpenApiRouter::new()
        .routes(routes!(list::api))
        .route_layer(middleware::from_fn(assert_login));

    let update = OpenApiRouter::new()
        .routes(routes!(update::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_write_group))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    let delete = OpenApiRouter::new()
        .routes(routes!(delete::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_owner))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    let list_members = OpenApiRouter::new()
        .routes(routes!(list_members::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    let fetch_members_self = OpenApiRouter::new()
        .routes(routes!(fetch_members_self::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    let list_default_blacklist = OpenApiRouter::new()
        .routes(routes!(list_default_blacklist::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    let list_default_market = OpenApiRouter::new()
        .routes(routes!(list_default_market::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    OpenApiRouter::new()
        .merge(create)
        .merge(list)
        .merge(update)
        .merge(delete)
        .merge(fetch)
        .merge(list_members)
        .merge(fetch_members_self)
        .merge(list_default_blacklist)
        .merge(list_default_market)
}

starfoundry_uuid!(ProjectGroupUuid, "ProjectGroupUuid");

async fn assert_exists(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    request:                  Request,
    next:                     Next,
) -> Result<impl IntoResponse> {
    permission::assert_exists(
            &state.pool,
            project_group_uuid,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_read(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    identity:                 ExtractIdentity,
    request:                  Request,
    next:                     Next,
) -> Result<impl IntoResponse> {
    permission::assert_read_access(
            &state.pool,
            project_group_uuid,
            identity.character_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_write_group(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    identity:                 ExtractIdentity,
    request:                  Request,
    next:                     Next,
) -> Result<impl IntoResponse> {
    permission::assert_write_access(
            &state.pool,
            project_group_uuid,
            identity.character_id,
            permission::ProjectGroupPermissionCode::WriteGroup,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_owner(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    identity:                 ExtractIdentity,
    request:                  Request,
    next:                     Next,
) -> Result<impl IntoResponse> {
    permission::assert_write_access(
            &state.pool,
            project_group_uuid,
            identity.character_id,
            permission::ProjectGroupPermissionCode::Owner,
        )
        .await?;

    Ok(next.run(request).await)
}

#[cfg(test)]
pub async fn project_group_test_routes(
    pool: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use starfoundry_lib_eve_gateway::test::set_jwt_test_envs;
    use tower::ServiceExt;

    let state = AppState {
        pool: pool.clone(),
    };
    let (app, _) = crate::project_group::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    set_jwt_test_envs();

    app
        .clone()
        .oneshot(request)
        .await
        .unwrap()
}
