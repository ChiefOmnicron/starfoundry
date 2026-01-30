mod middleware;
mod setup;

pub use self::middleware::*;
pub use self::setup::*;

use axum::http::header::CONTENT_TYPE;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use metrics_exporter_prometheus::PrometheusHandle;

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
