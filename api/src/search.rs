use sqlx::PgPool;
use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;

pub mod error;
pub use self::error::*;
use crate::with_pool;

mod systems;

pub mod service {
    pub use super::systems::*;
}

use self::systems::SystemQuery;

pub fn api(
    pool:      PgPool,
    base_path: BoxedFilter<()>,
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .clone()
        .and(warp::path!("search" / ..))
        .and(with_pool(pool.clone()))
        .boxed();

    // TODO: doc
    let systems = path
        .clone()
        .and(warp::path!("systems"))
        .and(warp::get())
        .and(warp::query())
        .and_then(systems)
        .boxed();

    systems
}

async fn systems(
    pool:  PgPool,
    query: SystemQuery,
) -> Result<impl Reply, Rejection> {
    self::service::systems(&pool, query)
        .await
        .map_err(Into::into)
        .map(|x| warp::reply::json(&x))
}
