//mod can_write;
//mod create;
//mod delete;
mod error;
//mod fetch_default_blacklist;
//mod fetch_default_market;
//mod fetch_members;
mod fetch;
mod list;
mod permission;
//mod update;

use axum::extract::{Path, Request, State};
use axum::middleware;
use axum::middleware::Next;
use axum::response::IntoResponse;
use starfoundry_libs_types::starfoundry_uuid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::auth::{assert_login, ExtractIdentity};
use crate::project_group::error::Result;

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new()
        .routes(routes!(list::api));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    OpenApiRouter::new()
        .merge(list)
        .merge(fetch)
}

starfoundry_uuid!(ProjectGroupUuid, "ProjectGroupUuid");

async fn assert_exists(
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    permission::assert_exists(
            &state.pool,
            project_group_uuid,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_read(
    State(state): State<AppState>,
    Path(project_group_uuid): Path<ProjectGroupUuid>,
    ExtractIdentity(identity): ExtractIdentity,
    request: Request,
    next: Next,
) -> Result<impl IntoResponse> {
    permission::assert_read_access(
            &state.pool,
            project_group_uuid,
            identity.character_id(),
        )
        .await?;

    Ok(next.run(request).await)
}
