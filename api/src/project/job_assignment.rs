mod create;
mod fetch;
mod update_job_state;

pub use self::create::*;
pub use self::fetch::*;
pub use self::update_job_state::*;

use sqlx::PgPool;
use starfoundry_libs_eve_api::Credentials;
use starfoundry_libs_projects::{ProjectJobAssignmentUuid, ProjectJobUuid};
use utoipa::IntoParams;
use warp::Filter;
use warp::filters::BoxedFilter;
use warp::reply::Reply;

use crate::{with_identity, with_pool};

pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .and(with_pool(pool.clone()))
        .and(warp::path!("projects" / "job-assignments" / ..));

    let fetch = path
        .clone()
        .and(warp::path!(ProjectJobAssignmentUuid))
        .and(warp::get())
        .and_then(fetch);

    let create = path
        .clone()
        .and(with_identity(pool.clone(), credentials))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(create);

    let update = path
        .clone()
        .and(warp::path!(ProjectJobAssignmentUuid / ProjectJobUuid / "state"))
        .and(warp::put())
        .and_then(update_job_state);

    fetch
        .or(create)
        .or(update)
        .boxed()
}

#[derive(IntoParams)]
#[into_params(names("projectJobAssignmentUuid"))]
pub struct ProjectJobAssignmentUuidPath(pub ProjectJobAssignmentUuid);
