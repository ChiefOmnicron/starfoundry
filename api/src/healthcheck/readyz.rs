use warp::{Reply, Rejection};

/// /health/livez
///
/// Deeper check if the service is still available
/// 
#[utoipa::path(
    get,
    operation_id = "health_livez",
    path = "/health/livez",
    tag = "healthcheck",
    responses(
        (
            description = "Everything is a-okay",
            status = OK,
        ),
    ),
)]
pub async fn livez() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_header(
        warp::reply::with_status(
            warp::reply::json(&String::from("healthy")),
            warp::http::StatusCode::OK,
        ),
        "Cache-Control",
        "no-cache"
    ))
}
