use sqlx::PgPool;
use starfoundry_libs_eve_api::CredentialCache;
use starfoundry_libs_projects::ProjectUuid;
use std::sync::{Arc, Mutex};
use utoipa::IntoParams;
use warp::{Filter, Reply};
use warp::filters::BoxedFilter;

use crate::{with_identity, with_pool};

mod check_resources;
mod cost_estimate;
mod create;
mod delete;
mod fetch;
mod list;
mod models;
mod update;

pub mod excess;
pub mod job_assignment;
pub mod job;
pub mod market;
pub mod misc;
pub mod product;
pub mod permission;
pub mod stock;

pub mod service {
    pub use super::check_resources::*;
    pub use super::cost_estimate::*;
    pub use super::create::*;
    pub use super::delete::*;
    pub use super::fetch::*;
    pub use super::list::*;
    pub use super::update::*;
}

pub fn api(
    pool:             PgPool,
    base_path:        BoxedFilter<()>,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> BoxedFilter<(impl Reply,)> {
    let path = base_path
        .clone()
        .and(warp::path!("projects" / ..))
        .and(with_pool(pool.clone()));

    let fetch = path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectUuid))
        .and(warp::get())
        .and_then(service::fetch);

    let check_resources = path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!("check"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(service::check_resources);

    let cost_estimate = path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!("cost-estimate"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(service::cost_estimate);

    let create = path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path::end())
        .and(warp::post())
        .and(warp::body::json())
        .and_then(service::create);

    let delete = path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectUuid))
        .and(warp::delete())
        .and_then(service::delete);

    let list = path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path::end())
        .and(warp::get())
        .and(warp::query())
        .and_then(service::list);

    let update = path
        .clone()
        .and(with_identity(pool.clone(), credential_cache.clone()))
        .and(warp::path!(ProjectUuid))
        .and(warp::put())
        .and(warp::body::json())
        .and_then(service::update);

    let root = fetch
        .or(check_resources)
        .or(cost_estimate)
        .or(create)
        .or(delete)
        .or(list)
        .or(update)
        .boxed();

    root
        .or(excess::api(
            pool.clone(),
            base_path.clone(),
            credential_cache.clone(),
        ))
        .or(job::api(
            pool.clone(),
            base_path.clone(),
            credential_cache.clone(),
        ))
        .or(job_assignment::api(
            pool.clone(),
            base_path.clone(),
            credential_cache.clone(),
        ))
        .or(permission::api(
            pool.clone(),
            base_path.clone(),
            credential_cache.clone(),
        ))
        .or(market::api(
            pool.clone(),
            base_path.clone(),
            credential_cache.clone(),
        ))
        .or(misc::api(
            pool.clone(),
            base_path.clone(),
            credential_cache.clone(),
        ))
        .or(product::api(
            pool.clone(),
            base_path.clone(),
            credential_cache.clone(),
        ))
        .or(stock::api(
            pool.clone(),
            base_path.clone(),
            credential_cache.clone(),
        ))
        .boxed()
}

#[derive(IntoParams)]
#[into_params(
    names("projectUuid"),
    parameter_in = Path,
)]
pub struct ProjectUuidPath(pub ProjectUuid);
