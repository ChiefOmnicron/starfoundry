mod export;
mod registry;

use prometheus_client::registry::Registry;
use std::convert::Infallible;
use std::sync::Arc;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

pub use self::registry::*;

pub fn api(
    registry: Arc<Registry>,
) -> BoxedFilter<(impl Reply,)> {
    let metric = warp::path!("metrics")
        .and(warp::get())
        .and(with_registry(registry))
        .and_then(export::export)
        .boxed();

    metric
}

fn with_registry(
    registry: Arc<Registry>,
) -> impl Filter<Extract = (Arc<Registry>,), Error = Infallible> + Clone {
    warp::any().map(move || registry.clone())
}

pub fn with_metric(
    metric: Arc<Metric>,
) -> impl Filter<Extract = (Arc<Metric>,), Error = Infallible> + Clone {
    warp::any().map(move || metric.clone())
}
