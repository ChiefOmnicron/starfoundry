#![allow(
    clippy::redundant_field_names
)]

use sqlx::PgPool;
use starfoundry_libs_eve_api::CredentialCache;
use std::convert::Infallible;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use warp::{Filter, Reply};
use warp::filters::path::FullPath;
use warp::reply::Response;

pub mod api_docs;
pub mod appraisal;
pub mod auth;
pub mod character;
pub mod config;
pub mod corporation;
pub mod error;
pub mod feature_flags;
pub mod healthcheck;
pub mod industry;
pub mod item;
pub mod job_detection;
pub mod notification;
pub mod project;
pub mod project_group;
pub mod search;
pub mod stock;
pub mod structure_dynamic_group;
pub mod structure_group;
pub mod structure;
pub mod version;

pub use self::error::*;

pub use self::auth::{with_identity, Identity};

pub fn with_pool(
    pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn with_credential_cache(
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> impl Filter<Extract = (Arc<Mutex<CredentialCache>>,), Error = Infallible> + Clone {
    warp::any().map(move || credential_cache.clone())
}

pub fn metric_wrapper<F, T>(
    filter: F,
) -> impl Filter<Extract = (Response,), Error = Infallible> + Clone + Send + Sync + 'static
where
    F: Filter<Extract = (T,), Error = Infallible> + Clone + Send + Sync + 'static,
    T: Reply,
{
    warp::any()
        .map(|| Instant::now())
        //.and(warp::filters::header::headers_cloned())
        .and(warp::filters::path::full())
        .and(filter)
        //.map(|start: Instant, headers: HeaderMap, path: FullPath, res: T| {
        .map(|start: Instant, path: FullPath, res: T| {
            let res = res.into_response();
            tracing::info!("status: {:?}", &res.status());
            tracing::info!("path: {:?}", path);
            tracing::info!("duration: {:?}", start.elapsed().as_millis());

            res
        })
}
