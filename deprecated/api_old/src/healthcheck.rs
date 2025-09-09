use sqlx::PgPool;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use crate::with_pool;

mod healthz;
mod readyz;

pub use self::healthz::*;
pub use self::readyz::*;

pub fn api(
    pool: PgPool,
) -> BoxedFilter<(impl Reply,)> {
    let livez = warp::path!("healthz")
        .and(warp::get())
        .and_then(healthz)
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
