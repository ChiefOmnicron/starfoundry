use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use starfoundry_lib_types::starfoundry_uuid;
use utoipa::IntoParams;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use crate::{with_identity, with_pool};

mod can_write;
mod create;
mod delete;
mod error;
mod fetch_default_blacklist;
mod fetch_default_market;
mod fetch_members;
mod fetch;
mod list;
mod permission;
mod update;

pub mod service {
    pub use super::can_write::*;
    pub use super::create::*;
    pub use super::delete::*;
    pub use super::fetch::*;
    pub use super::fetch_default_blacklist::*;
    pub use super::fetch_default_market::*;
    pub use super::fetch_members::*;
    pub use super::list::*;
    pub use super::update::*;
}

pub fn api(
    pool:             PgPool,
    base_path:        BoxedFilter<()>,
    credential_cache: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let group_path = base_path
        .clone()
        .and(warp::path!("project-groups" / ..))
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credential_cache.clone()));

    let create = group_path
        .clone()
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(service::create_api);

    let list = group_path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query())
        .and_then(service::list_api);

    let can_write = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "can-write"))
        .and(warp::get())
        .and_then(service::can_write_api);

    let fetch = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid))
        .and(warp::get())
        .and_then(service::fetch_api);

    let delete = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid))
        .and(warp::delete())
        .and_then(service::delete_api);

    let update = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(service::update_api);

    let fetch_default_blacklist = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "defaults" / "blacklist"))
        .and(warp::get())
        .and_then(service::fetch_default_blacklist_api);

    let fetch_default_market = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "defaults" / "markets"))
        .and(warp::get())
        .and_then(service::fetch_default_market_api);

    /*let update_default = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "defaults"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(service::update_default_api);

    let accept_invite = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "members" / "invite"))
        .and(warp::put())
        .and_then(service::accept_invite_api);

    let accept_member = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "members" / CharacterId / "accept"))
        .and(warp::put())
        .and_then(service::accept_member_api);

    let fetch_members = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "members"))
        .and(warp::get())
        .and_then(service::fetch_members_api);

    let remove_member = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "members" / CharacterId))
        .and(warp::delete())
        .and_then(service::remove_member_api);

    let update_member = group_path
        .clone()
        .and(warp::path!(ProjectGroupUuid / "members" / CharacterId))
        .and(warp::body::json())
        .and(warp::put())
        .and_then(service::update_member_api);*/

    create
        .or(list)
        .or(can_write)
        .or(fetch)
        .or(delete)
        .or(update)
        .or(fetch_default_blacklist)
        .or(fetch_default_market)
        //.or(update_default)
        //.or(accept_invite)
        //.or(accept_member)
        //.or(fetch_members)
        //.or(remove_member)
        //.or(update_member)
        .boxed()
}

#[derive(IntoParams)]
#[into_params(names("projectGroupUuid"))]
pub struct ProjectGroupUuidPath(pub ProjectGroupUuid);

starfoundry_uuid!(ProjectGroupUuid, "ProjectGroupUuid");
