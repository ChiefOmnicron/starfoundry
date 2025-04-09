use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use std::sync::Arc;
use warp::http::Response;
use warp::reject::Rejection;
use warp::reply::Reply;

use super::Metric;

pub async fn export(
    registry: Arc<Registry>,
    metric:   Arc<Metric>,
) -> Result<impl Reply, Rejection> {
    let mut buffer = String::new();
    encode(&mut buffer, &registry).unwrap();

    let response = Response::builder()
        .header("Content-Type", "text/plain; version=0.0.4")
        .body(buffer);

    metric.reset_appraisal_metrics();
    Ok(response)
}
