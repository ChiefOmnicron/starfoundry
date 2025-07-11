use sqlx::PgPool;
use starfoundry_libs_structures::{StructureDynamicGroup, StructureDynamicGroupService, StructureDynamicGroupUuid};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};

pub async fn update(
    pool:       PgPool,
    identity:   Identity,
    group_uuid: StructureDynamicGroupUuid,
    group:      StructureDynamicGroup,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    let group_id = StructureDynamicGroupService::new(group_uuid).update(
            &pool,
            character_id,
            group,
        )
        .await
        .map_err(|_| ReplyError::Unauthorized)?;

    let response = warp::reply::with_status(
        warp::reply::json(&group_id),
        StatusCode::NO_CONTENT,
    );

    Ok(response)
}
