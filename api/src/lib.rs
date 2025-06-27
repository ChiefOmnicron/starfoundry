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
use warp::http::HeaderMap;
use warp::reject::Rejection;
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
pub mod metric;
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

use crate::metric::{with_metric, Metric};

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
    metric: Arc<Metric>,
) -> impl Filter<Extract = (Response,), Error = Infallible> + Clone + Send + Sync + 'static
where
    F: Filter<Extract = (T,), Error = Infallible> + Clone + Send + Sync + 'static,
    T: Reply,
{
    warp::any()
        .map(|| Instant::now())
        .and(warp::header::headers_cloned())
        .and(warp::filters::path::full())
        .and(warp::method())
        .and(with_metric(metric.clone()))
        .and(filter)
        .map(|duration: Instant, headers: HeaderMap, path: FullPath, method: warp::http::Method, metric: Arc<Metric>, res: T| {
            let res = res.into_response();

            let agent = headers
                .get(warp::http::header::USER_AGENT)
                .map(|x| x.to_str().unwrap_or("Unknown"))
                .unwrap_or("Unknown");
            {
                metric
                    .inc_route_count(
                        &method,
                        &res.status(),
                        agent,
                        path.as_str(),
                    );
                metric
                    .add_route_duration(
                        &method,
                        &res.status(),
                        agent,
                        path.as_str(),
                        duration.elapsed().as_secs_f64()
                    );
            }

            res
        })
}
