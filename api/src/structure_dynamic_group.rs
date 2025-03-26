use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_structures::StructureDynamicGroupUuid;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{with_identity, with_pool};

mod create;
mod delete;
mod fetch;
mod list;
mod update;

pub mod service {
    pub use super::create::*;
    pub use super::delete::*;
    pub use super::fetch::*;
    pub use super::list::*;
    pub use super::update::*;
}

pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("structures" / "groups" / "dynamic" / ..))
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()))
        .boxed();

    // TODO: doc
    let list = base_path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(service::list)
        .boxed();

    // TODO: doc
    let by_id = base_path
        .clone()
        .and(warp::path!(StructureDynamicGroupUuid))
        .and(warp::get())
        .and_then(service::by_id)
        .boxed();

    // TODO: doc
    let create = base_path
        .clone()
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(service::create)
        .boxed();

    // TODO: doc
    let delete = base_path
        .clone()
        .and(warp::path!(StructureDynamicGroupUuid))
        .and(warp::delete())
        .and_then(service::delete)
        .boxed();

    // TODO: doc
    let update = base_path
        .clone()
        .and(warp::path!(StructureDynamicGroupUuid))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(service::update)
        .boxed();

    list
        .or(by_id)
        .or(create)
        .or(delete)
        .or(update)
        .boxed()
}
