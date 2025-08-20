use sqlx::PgPool;
use warp::{Reply, Rejection};

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
