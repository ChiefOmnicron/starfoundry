use sqlx::PgPool;
use starfoundry_lib_structures::StructureDynamicGroupService;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};

pub async fn list(
    pool:     PgPool,
    identity: Identity,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    let ids = StructureDynamicGroupService::list(
            &pool,
            character_id,
        )
        .await
        .map_err(|_| ReplyError::Unauthorized)?;

    let response = if ids.is_empty() {
        warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )
    } else {
        warp::reply::with_status(
            warp::reply::json(&ids),
            StatusCode::OK,
        )
    };

    Ok(response)
}
