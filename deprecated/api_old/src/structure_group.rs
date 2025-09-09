use sqlx::PgPool;
use starfoundry_lib_eve_api::CredentialCache;
use starfoundry_lib_structures::StructureGroupUuid;
use std::sync::{Arc, Mutex};
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{with_identity, with_pool};

mod fetch;
mod create;
mod delete;
mod list;

pub mod service {
    pub use super::fetch::*;
    pub use super::create::*;
    pub use super::delete::*;
    pub use super::list::*;
}

pub fn api(
    pool:             PgPool,
    base_path:        BoxedFilter<()>,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("structures" / "groups" / ..))
        .and(with_pool(pool.clone()))
        .boxed();

    // TODO: doc
    let list = base_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path::end())
        .and(warp::get())
        .and_then(service::list)
        .boxed();

    // TODO: doc
    let by_id = base_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(StructureGroupUuid))
        .and(warp::get())
        .and_then(service::fetch)
        .boxed();

    // TODO: doc
    let create = base_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(service::create)
        .boxed();

    // TODO: doc
    let delete = base_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(StructureGroupUuid))
        .and(warp::delete())
        .and_then(service::delete)
        .boxed();

    list
        .or(by_id)
        .or(create)
        .or(delete)
        .boxed()
}
