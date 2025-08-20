mod add;
mod delete;
mod fetch;
mod update;

pub use self::add::*;
pub use self::delete::*;
pub use self::fetch::*;
pub use self::update::*;

use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use starfoundry_lib_projects::{ProjectUuid, ProjectMiscUuid};
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
        .and(warp::path!("projects" / ProjectUuid / "misc" / ..));

    let add = path
        .clone()
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(add);

    let delete = path
        .clone()
        .and(warp::path!(ProjectMiscUuid))
        .and(warp::delete())
        .and_then(delete);

    let fetch = path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(fetch);

    let update = path
        .clone()
        .and(warp::path::end())
        .and(warp::put())
        .and(warp::body::json())
        .and_then(update);

    add
        .or(delete)
        .or(fetch)
        .or(update)
        .boxed()
}

#[derive(IntoParams)]
#[into_params(names("ProjectMiscUuid"))]
pub struct ProjectMiscUuidPath(pub ProjectMiscUuid);
