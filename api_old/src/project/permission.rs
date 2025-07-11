mod can_write;
mod is_owner;

pub use self::can_write::*;
pub use self::is_owner::*;

use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_projects::ProjectUuid;
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
        .and(warp::path!("projects" / ProjectUuid / "permissions" / ..));

    let can_write = path
        .clone()
        .and(warp::path!("can-write"))
        .and(warp::get())
        .and_then(can_write);

    let is_owner = path
        .clone()
        .and(warp::path!("is-owner"))
        .and(warp::get())
        .and_then(is_owner);

    can_write
        .or(is_owner)
        .boxed()
}
