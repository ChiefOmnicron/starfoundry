use prometheus_client::encoding::text::encode;
use prometheus_client::registry::Registry;
use sqlx::PgPool;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;
use warp::http::Response;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::filters::BoxedFilter;

pub async fn api(
    pool:        PgPool,
    registry:    Registry,
    socket_addr: SocketAddr,
) {
    let metric = warp::path!("metrics")
        .and(warp::get())
        .and(with_registry(Arc::new(registry)))
        .and_then(metrics)
        .boxed();

    let routes = healthcheck(pool.clone())
        .or(metric);

    tracing::info!("Starting service server on {}", socket_addr);
    warp::serve(routes).run(socket_addr).await;
}

pub fn healthcheck(
    pool: PgPool,
) -> BoxedFilter<(impl Reply,)> {
    let livez = warp::path!("healthz")
        .and(warp::get())
        .and_then(livez)
        .boxed();

    let readyz = warp::path!("readyz")
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and_then(readyz)
        .boxed();

    livez
        .or(readyz)
        .boxed()
}

async fn metrics(
    registry: Arc<Registry>,
) -> Result<impl Reply, Rejection> {
    let mut buffer = String::new();
    encode(&mut buffer, &registry).unwrap();

    let response = Response::builder()
        .header("Content-Type", "text/plain; version=0.0.4")
        .body(buffer);
    Ok(response)
}

async fn livez() -> Result<impl Reply, Rejection> {
    Ok(warp::reply::with_header(
        warp::reply::with_status(
            warp::reply::json(&String::from("healthy")),
            warp::http::StatusCode::OK,
        ),
        "Cache-Control",
        "no-cache"
    ))
}

async fn readyz(
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

fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

fn with_registry(
    registry: Arc<Registry>,
) -> impl Filter<Extract = (Arc<Registry>,), Error = Infallible> + Clone {
    warp::any().map(move || registry.clone())
}
