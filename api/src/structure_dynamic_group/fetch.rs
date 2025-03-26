use sqlx::PgPool;
use starfoundry_libs_structures::{StructureDynamicGroupService, StructureDynamicGroupUuid};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};

pub async fn by_id(
    pool:     PgPool,
    identity: Identity,
    group_id: StructureDynamicGroupUuid,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    let group = StructureDynamicGroupService::new(group_id).fetch(
            &pool,
            character_id,
        )
        .await
        .map_err(|_| ReplyError::Unauthorized)?;

    let response = if group.is_some() {
        warp::reply::with_status(
            warp::reply::json(&group),
            StatusCode::OK,
        )
    } else {
        warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NOT_FOUND,
        )
    };

    Ok(response)
}
