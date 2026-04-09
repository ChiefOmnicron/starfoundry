mod error;
mod bulk;
mod last_fetch;
mod service;
mod virtual_market;

pub use self::service::*;

use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

use crate::AppState;

pub fn routes(
    state: AppState,
) -> OpenApiRouter<AppState> {
    let bulk = OpenApiRouter::new()
        .routes(routes!(bulk::api));

    let last_fetch = OpenApiRouter::new()
        .routes(routes!(last_fetch::api));

    let virtual_market = OpenApiRouter::new()
        .routes(routes!(virtual_market::api));

    OpenApiRouter::new()
        .merge(bulk)
        .merge(last_fetch)
        .merge(virtual_market)
}

#[cfg(test)]
pub async fn market_test_routes(
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
    let (app, _) = crate::market::routes(state.clone()).split_for_parts();
    let app = app.with_state(state.clone());

    app
        .clone()
        .oneshot(request)
        .await
        .unwrap()
}
