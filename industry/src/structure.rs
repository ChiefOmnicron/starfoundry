mod create;
mod delete;
mod error;
mod fetch;
mod list;
mod permission;
mod update;

pub mod service;

pub use self::error::StructureError;

use axum::extract::{Path, Request, State};
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::starfoundry_uuid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::structure::error::Result;

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

starfoundry_uuid!(StructureUuid, "StructureUuid");

async fn assert_exists(
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureUuid>,
    request:            Request,
    next:               Next,
) -> Result<impl IntoResponse> {
    permission::assert_exists(
            &state.pool,
            structure_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_read(
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureUuid>,
    identity:           ExtractIdentity,
    request:            Request,
    next:               Next,
) -> Result<impl IntoResponse> {
    permission::assert_read_access(
            &state.pool,
            structure_id,
            identity.character_id,
        )
        .await?;

    Ok(next.run(request).await)
}

async fn assert_write(
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureUuid>,
    identity:           ExtractIdentity,
    request:            Request,
    next:               Next,
) -> Result<impl IntoResponse> {
    permission::assert_write_access(
            &state.pool,
            structure_id,
            identity.character_id,
        )
        .await?;

    Ok(next.run(request).await)
}

#[cfg(test)]
pub async fn structure_test_routes(
    pool: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use tower::ServiceExt;

    let state: AppState = AppState {
        pool: pool.clone(),
    };
    let (app, _) = crate::structure::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app
        .clone()
        .oneshot(request)
        .await
        .unwrap()
}
