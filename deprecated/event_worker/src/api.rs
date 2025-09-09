mod healthcheck;
mod metric;

use prometheus_client::registry::Registry;
use sqlx::PgPool;
use std::convert::Infallible;
use std::net::SocketAddr;
use std::sync::Arc;
use warp::Filter;

pub async fn api(
    pool:     PgPool,
    registry: Registry,
) {
    let metric = warp::path!("metrics")
        .and(warp::get())
        .and(with_registry(Arc::new(registry)))
        .and_then(metric::api)
        .boxed();

    let routes = healthcheck::api(pool.clone())
        .or(metric);

    warp::serve(routes).run(address()).await;
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

fn address() -> SocketAddr {
    if let Ok(x) = std::env::var("SERVER_ADDRESS") {
        x.parse().unwrap()
    } else {
        tracing::error!("Missing ENV 'SERVER_ADDRESS'");
        "0.0.0.0:21000".parse().unwrap()
    }
}
