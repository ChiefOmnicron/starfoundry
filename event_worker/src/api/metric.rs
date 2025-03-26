use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use std::sync::Arc;
use warp::http::Response;
use warp::reject::Rejection;
use warp::reply::Reply;

pub async fn api(
    registry: Arc<Registry>,
) -> Result<impl Reply, Rejection> {
    let mut buffer = String::new();
    encode(&mut buffer, &registry).unwrap();

    let response = Response::builder()
        .header("Content-Type", "")
        .body(buffer);
    Ok(response)
}
