mod error;
mod fetch;
mod list;
mod permission;

pub use self::error::StructureError;
pub use self::fetch::Structure;

use axum::extract::{Path, Request, State};
use axum::middleware::{self, Next};
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{assert_login, ExtractIdentity};
use starfoundry_lib_types::starfoundry_uuid;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::structure::error::Result;

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let list = OpenApiRouter::new()
        .routes(routes!(list::api))
        .route_layer(middleware::from_fn(assert_login));

    let fetch = OpenApiRouter::new()
        .routes(routes!(fetch::api))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_read))
        .route_layer(middleware::from_fn_with_state(state.clone(), assert_exists))
        .route_layer(middleware::from_fn(assert_login));

    OpenApiRouter::new()
        .merge(list)
        .merge(fetch)
}

starfoundry_uuid!(StructureUuid, "StructureUuid");

async fn assert_exists(
    State(state):             State<AppState>,
    Path(project_group_uuid): Path<StructureUuid>,
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
    Path(project_group_uuid): Path<StructureUuid>,
    identity:                 ExtractIdentity,
    request:                  Request,
    next:                     Next,
) -> Result<impl IntoResponse> {
    permission::assert_read_access(
            &state.pool,
            project_group_uuid,
            identity.character_info.character_id,
        )
        .await?;

    Ok(next.run(request).await)
}

#[cfg(test)]
pub async fn structure_test_routes(
    pool: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use starfoundry_lib_eve_gateway::test::{decoding_key, set_jwt_test_envs};
    use std::sync::Arc;
    use tower::ServiceExt;

    let state = AppState {
        pool: pool.clone(),

        decoding_key: Arc::new(decoding_key()),
    };
    let (app, _) = crate::structure::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    set_jwt_test_envs();

    app
        .clone()
        .oneshot(request)
        .await
        .unwrap()
}
