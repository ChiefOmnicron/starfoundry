mod error;
mod bulk;
mod service;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;
use crate::market::error::Result;

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let bulk = OpenApiRouter::new()
        .routes(routes!(bulk::api));

    OpenApiRouter::new()
        .merge(bulk)
}

#[cfg(test)]
pub async fn market_test_routes(
    pool: sqlx::PgPool,
    request: axum::http::Request<axum::body::Body>,
) -> axum::http::Response<axum::body::Body> {
    use tower::ServiceExt;

    let state = AppState {
        pool: pool.clone(),
    };
    let (app, _) = crate::market::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app
        .clone()
        .oneshot(request)
        .await
        .unwrap()
}
