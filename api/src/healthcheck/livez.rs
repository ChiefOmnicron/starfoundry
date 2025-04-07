use sqlx::PgPool;
use warp::{Reply, Rejection};

use crate::api_docs::InternalServerError;

/// /health/readyz
///
/// Checks if the servic is ready to accept connections
/// 
#[utoipa::path(
    get,
    operation_id = "health_readyz",
    path = "/health/readyz",
    tag = "healthcheck",
    responses(
        (
            description = "Everything is a-okay",
            status = OK,
        ),
        InternalServerError,
    ),
)]
pub async fn readyz(
    pool: PgPool,
) -> Result<impl Reply, Rejection> {
    let postgres_version = sqlx::query!("SELECT version()")
        .fetch_one(&pool)
        .await;

    if postgres_version.is_err() {
        return Ok(warp::reply::with_header(
            warp::reply::with_status(
                warp::reply::json(&String::from("unhealthy")),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ),
            "Cache-Control",
            "no-cache"
        ));
    }

    Ok(warp::reply::with_header(
        warp::reply::with_status(
            warp::reply::json(&String::from("healthy")),
            warp::http::StatusCode::OK,
        ),
        "Cache-Control",
        "no-cache"
    ))
}
