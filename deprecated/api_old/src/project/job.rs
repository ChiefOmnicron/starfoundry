mod active;
mod delete;
mod fetch;
mod startable;
mod update;

pub use self::active::*;
pub use self::delete::*;
pub use self::fetch::*;
pub use self::startable::*;
pub use self::update::*;

use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use starfoundry_lib_projects::{ProjectUuid, ProjectJobUuid};
use utoipa::IntoParams;
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
        .and(warp::path!("projects" / ProjectUuid / "jobs" / ..));

    let active = path
        .clone()
        .and(warp::path!("active"))
        .and(warp::get())
        .and_then(active);

    let delete = path
        .clone()
        .and(warp::path!(ProjectJobUuid))
        .and(warp::delete())
        .and_then(delete);

    let fetch = path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query())
        .and_then(fetch);

    let startable = path
        .clone()
        .and(warp::path!("startable"))
        .and(warp::get())
        .and_then(startable);

    let update = path
        .clone()
        .and(warp::path!(ProjectJobUuid))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update);

    active
        .or(delete)
        .or(fetch)
        .or(startable)
        .or(update)
        .boxed()
}

#[derive(IntoParams)]
#[into_params(names("projectJobUuid"))]
pub struct ProjectJobUuidPath(pub ProjectJobUuid);
