use sqlx::PgPool;
use starfoundry_lib_structures::{StructureGroupService, StructureGroupUuid};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};

pub async fn fetch(
    pool:       PgPool,
    identity:   Identity,
    group_uuid: StructureGroupUuid,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    let structure_group = StructureGroupService::new(group_uuid).fetch(
            &pool,
            character_id,
        )
        .await;

    if let Ok(x) = structure_group {
        let response = warp::reply::with_status(
            warp::reply::json(&x),
            StatusCode::OK,
        );
        Ok(response)
    } else {
        Err(ReplyError::Unauthorized.into())
    }
}
