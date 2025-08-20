mod middleware;
mod setup;

pub use self::middleware::*;
pub use self::setup::*;

use metrics_exporter_prometheus::PrometheusHandle;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::http::header::CONTENT_TYPE;

pub fn route(
    metrics: PrometheusHandle,
) -> impl IntoResponse {
    (
        StatusCode::OK,
        [(
            CONTENT_TYPE,
            "text/plain; version=0.0.4"
        )],
        metrics.render(),
    )
}
