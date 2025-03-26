use sqlx::PgPool;
use starfoundry_libs_structures::StructureGroupService;
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};

pub async fn list(
    pool:     PgPool,
    identity: Identity,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    let ids = StructureGroupService::list(
            &pool,
            character_id
        )
        .await;

    if let Ok(x) = ids {
        let response = if x.is_empty() {
            warp::reply::with_status(
                warp::reply::json(&()),
                StatusCode::NO_CONTENT,
            )
        } else {
            warp::reply::with_status(
                warp::reply::json(&x),
                StatusCode::OK,
            )
        };
        Ok(response)
    } else {
        Err(ReplyError::Unauthorized.into())
    }
}
