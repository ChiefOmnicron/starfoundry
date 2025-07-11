mod compression;
mod create;
mod fetch;
mod markets;
mod reprocessing;

pub use self::compression::*;
pub use self::create::*;
pub use self::fetch::*;
pub use self::markets::*;
pub use self::reprocessing::*;

use sqlx::PgPool;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::with_pool;
use crate::metric::{with_metric, WithMetric};

pub fn api(
    pool:      PgPool,
    metric:    WithMetric,
    base_path: BoxedFilter<()>,
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .clone()
        .and(warp::path!("appraisals" / ..))
        .and(with_pool(pool.clone()));

    let fetch = path
        .clone()
        .and(with_metric(metric.clone()))
        .and(warp::path!(String))
        .and(warp::get())
        .and_then(fetch);

    let create = path
        .clone()
        .and(with_metric(metric.clone()))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create);

    let markets = path
        .clone()
        .and(with_metric(metric.clone()))
        .and(warp::path!("markets"))
        .and(warp::get())
        .and_then(markets);

    let compression = path
        .clone()
        .and(with_metric(metric.clone()))
        .and(warp::path!(String / "compression"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(compression);

    let reprocessing = path
        .clone()
        .and(with_metric(metric.clone()))
        .and(warp::path!(String / "reprocessing"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(reprocessing);

    compression
        .or(create)
        .or(fetch)
        .or(markets)
        .or(reprocessing)
        .boxed()
}

