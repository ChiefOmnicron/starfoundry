mod fetch;
mod update_price;

pub use self::fetch::*;
pub use self::update_price::*;

use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use starfoundry_lib_projects::ProjectUuid;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use crate::{with_pool, with_identity};

pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::path!("projects" / ProjectUuid / "stocks" / ..));

    let fetch = path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(fetch);

    let update = path
        .clone()
        .and(warp::path!("prices"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_price);

    fetch
        .or(update)
        .boxed()
}
