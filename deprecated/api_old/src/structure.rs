use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use starfoundry_lib_structures::StructureUuid;
use starfoundry_lib_types::{StructureId, TypeId};
use utoipa::IntoParams;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{with_identity, with_pool};

pub mod create;
pub mod delete;
pub mod fetch;
pub mod list;
pub mod permission;
pub mod resolve_player_structure;
pub mod rigs;
pub mod update;

pub mod service {
    pub use super::create::*;
    pub use super::delete::*;
    pub use super::fetch::*;
    pub use super::list::*;
    pub use super::permission::*;
    pub use super::resolve_player_structure::*;
    pub use super::rigs::*;
    pub use super::update::*;
}

/// Filters that build up the api for this part of the application
pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .clone()
        .and(warp::path!("structures" / ..))
        .and(with_pool(pool.clone()))
        .boxed();

    let list = path
        .clone()
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query())
        .and_then(service::list)
        .boxed();

    let fetch = path
        .clone()
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::path!(StructureUuid))
        .and(warp::get())
        .and_then(service::fetch)
        .boxed();

    let create = path
        .clone()
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(service::create)
        .boxed();

    let update = path
        .clone()
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::path!(StructureUuid))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(service::update)
        .boxed();

    let delete = path
        .clone()
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::path!(StructureUuid))
        .and(warp::delete())
        .and_then(service::delete)
        .boxed();

    // Lookup in the eve api for the name
    let resolve_player_structure = path
        .clone()
        .and(with_identity(pool.clone(), credentials.clone()))
        .and(warp::path!(StructureId / "resolve"))
        .and(warp::get())
        .and_then(service::resolve_player_structure)
        .boxed();

    let rigs = path
        .clone()
        .and(warp::path!(TypeId / "rigs"))
        .and(warp::get())
        .and_then(service::rig_by_structure_type_id)
        .boxed();

    list
        .or(create)
        .or(delete)
        .or(fetch)
        .or(update)
        .or(resolve_player_structure)
        .or(rigs)
        .or(
            permission::api(
                pool,
                base_path,
                credentials,
            )
        )
        .boxed()
}

#[derive(IntoParams)]
#[into_params(names("structureUuid"))]
pub struct StructureUuidPath(pub StructureUuid);
