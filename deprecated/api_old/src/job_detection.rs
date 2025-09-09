mod fetch;
mod update_job_add;
mod update_job_delete;
mod update_job_replace;

pub use self::fetch::*;
pub use self::update_job_add::*;
pub use self::update_job_delete::*;
pub use self::update_job_replace::*;

use sqlx::PgPool;
use starfoundry_lib_eve_api::Credentials;
use starfoundry_lib_types::JobId;
use utoipa::IntoParams;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{with_identity, with_pool};

pub fn api(
    pool:        PgPool,
    base_path:   BoxedFilter<()>,
    credentials: Credentials,
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .clone()
        .and(warp::path!("job-detection" / ..))
        .and(with_pool(pool.clone()))
        .and(with_identity(pool.clone(), credentials.clone()));

    let fetch = path
        .clone()
        .and(warp::path::end())
        .and(warp::get())
        .and_then(self::fetch);

    let update_job_add = path
        .clone()
        .and(warp::path!(JobId / "add"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(self::update_job_add);

    let update_job_delete = path
        .clone()
        .and(warp::path!(JobId / "delete"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(self::update_job_delete);

    let update_job_replace = path
        .clone()
        .and(warp::path!(JobId / "replace"))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(self::update_job_replace);

    fetch
        .or(update_job_add)
        .or(update_job_delete)
        .or(update_job_replace)
        .boxed()
}

#[derive(IntoParams)]
#[into_params(
    names("jobId"),
    parameter_in = Path,
)]
pub struct JobIdPath(pub JobId);
