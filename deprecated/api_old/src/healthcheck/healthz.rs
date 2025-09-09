use warp::{Reply, Rejection};

/// /healthz
///
/// Checks if the service is ready to accept connections
/// 
#[utoipa::path(
    get,
    operation_id = "healthz",
    path = "/healthz",
    tag = "healthcheck",
    responses(
        (
            description = "Everything is a-okay",
            status = OK,
        ),
    ),
)]
pub async fn healthz() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_header(
        warp::reply::with_status(
            warp::reply::json(&String::from("healthy")),
            warp::http::StatusCode::OK,
        ),
        "Cache-Control",
        "no-cache"
    ))
}
