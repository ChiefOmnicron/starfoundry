use sqlx::PgPool;
use starfoundry_libs_eve_api::CredentialCache;
use starfoundry_libs_types::CorporationId;
use std::sync::{Arc, Mutex};
use warp::{Filter, Reply, Rejection};
use warp::filters::BoxedFilter;
use warp::http::StatusCode;

use crate::{with_identity, Identity};

pub mod error;
pub use self::error::*;

mod info;

pub mod service {
    pub use super::info::*;
}

pub fn api(
    pool:              PgPool,
    base_path:         BoxedFilter<()>,
    credential_cache:  Arc<Mutex<CredentialCache>>,
) -> BoxedFilter<(impl Reply,)> {
    let base_path = base_path
        .clone()
        .and(warp::path!("corporations" / ..))
        .and(with_identity(pool.clone(), credential_cache.clone()));

    // TODO: doc
    let info = base_path
        .clone()
        .and(warp::path!(CorporationId / "info"))
        .and(warp::get())
        .and_then(info)
        .boxed();
    info
}

async fn info(
    identity:       Identity,
    corporation_id: CorporationId,
) -> Result<impl Reply, Rejection> {
    let client = identity.api_client().await?;

    self::service::info(client, corporation_id)
        .await
        .map(|x| warp::reply::with_status(
            warp::reply::json(&x),
            StatusCode::OK
        ))
        .map_err(Into::into)
}
