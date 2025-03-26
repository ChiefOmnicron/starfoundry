use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_projects::ProjectGroupUuid;
use starfoundry_libs_types::CharacterId;
use utoipa::IntoParams;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use crate::{with_identity, with_pool};

mod accept_invite;
mod accept_member;
mod create;
mod delete;
mod fetch;
mod fetch_default;
mod fetch_members;
mod list;
mod remove_member;
mod update;
mod update_default;
mod update_member;

pub use self::accept_member::*;
pub use self::accept_invite::*;
pub use self::create::*;
pub use self::delete::*;
pub use self::fetch::*;
pub use self::fetch_default::*;
pub use self::fetch_members::*;
pub use self::list::*;
pub use self::remove_member::*;
pub use self::update::*;
pub use self::update_default::*;
pub use self::update_member::*;

pub fn api(
    pool:             PgPool,
    base_path:        BoxedFilter<()>,
    credential_cache: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let group_path = base_path
        .clone()
        .and(warp::path!("project-groups" / ..))
        .and(with_pool(pool.clone()));

    let create = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create);

    let list = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query())
        .and_then(list);

    let fetch = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid))
        .and(warp::get())
        .and_then(fetch);

    let delete = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid))
        .and(warp::delete())
        .and_then(delete);

    let update = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update);

    let fetch_default = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid / "default"))
        .and(warp::get())
        .and_then(fetch_default);

    let update_default = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid / "default"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update_default);

    let accept_invite = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid / "members" / "invite"))
        .and(warp::put())
        .and_then(accept_invite);

    let accept_member = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid / "members" / CharacterId / "accept"))
        .and(warp::put())
        .and_then(accept_member);

    let fetch_members = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid / "members"))
        .and(warp::get())
        .and_then(fetch_members);

    let remove_member = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid / "members" / CharacterId))
        .and(warp::delete())
        .and_then(remove_member);

    let update_member = group_path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectGroupUuid / "members" / CharacterId))
        .and(warp::body::json())
        .and(warp::put())
        .and_then(update_member);

    create
        .or(list)
        .or(fetch)
        .or(delete)
        .or(update)
        .or(fetch_default)
        .or(update_default)
        .or(accept_invite)
        .or(accept_member)
        .or(fetch_members)
        .or(remove_member)
        .or(update_member)
        .boxed()
}

#[derive(IntoParams)]
#[into_params(names("projectGroupUuid"))]
pub struct ProjectGroupUuidPath(pub ProjectGroupUuid);
