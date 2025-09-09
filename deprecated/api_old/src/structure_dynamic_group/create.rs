use sqlx::PgPool;
use starfoundry_lib_structures::{StructureDynamicGroup, StructureDynamicGroupService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::ReplyError;
use crate::Identity;

pub async fn create(
    pool:     PgPool,
    identity: Identity,
    group:    StructureDynamicGroup,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    let group_id = StructureDynamicGroupService::create(
            &pool,
            character_id,
            group,
        )
        .await
        .map_err(|_| ReplyError::Unauthorized)?;

    let response = warp::reply::with_status(
        warp::reply::json(&group_id),
        StatusCode::CREATED,
    );

    Ok(response)
}
