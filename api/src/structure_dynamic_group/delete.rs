use sqlx::PgPool;
use starfoundry_libs_structures::{StructureDynamicGroupService, StructureDynamicGroupUuid};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::ReplyError;
use crate::Identity;

pub async fn delete(
    pool:     PgPool,
    identity: Identity,
    group_id: StructureDynamicGroupUuid,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    let group_id = StructureDynamicGroupService::new(group_id).delete(
            &pool,
            character_id,
        )
        .await
        .map_err(|_| ReplyError::Unauthorized)?;

    let response = warp::reply::with_status(
        warp::reply::json(&group_id),
        StatusCode::OK,
    );

    Ok(response)
}
