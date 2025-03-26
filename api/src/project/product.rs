mod fetch;

pub use self::fetch::*;

use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_projects::ProjectUuid;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use crate::{with_identity, with_pool};

pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials))
        .and(warp::path!("projects" / ProjectUuid / "products" / ..));

    let fetch = path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(fetch);

    fetch
        .boxed()
}
