use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::with_pool;

mod error;
mod system_index;

pub mod service {
    pub use super::system_index::*;
}

/// Filters that build up the api for this part of the application
pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    _:           Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("industry" / ..))
        .boxed();

    let system_index = base_path
        .clone()
        .and(warp::path!("system-index"))
        .and(warp::get())
        .and(with_pool(pool.clone()))
        .and(warp::query())
        .and_then(service::system_index_api)
        .boxed();

    system_index
        .boxed()
}
