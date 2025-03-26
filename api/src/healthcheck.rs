use sqlx::PgPool;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use crate::with_pool;

mod livez;
mod readyz;

pub use self::livez::*;
pub use self::readyz::*;

pub fn api(
    pool:      PgPool,
) -> BoxedFilter<(impl Reply,)> {
    let path = warp::path!("health" / ..)
        .boxed();

    let livez = path
        .clone()
        .and(warp::path!("livez"))
        .and(warp::get())
        .and_then(livez)
        .boxed();

    let readyz = path
        .clone()
        .and(warp::path!("readyz"))
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and_then(readyz)
        .boxed();

    livez
        .or(readyz)
        .boxed()
}
