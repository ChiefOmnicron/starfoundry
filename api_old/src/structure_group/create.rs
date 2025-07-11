use sqlx::PgPool;
use starfoundry_libs_structures::{CreateGroup, StructureGroupService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};

pub async fn create(
    pool:            PgPool,
    identity:        Identity,
    structure_group: CreateGroup,
) -> Result<impl Reply, Rejection> {
    let character_id = identity.character_id();

    let tags = StructureGroupService::create(
        &pool,
        character_id,
        structure_group,
    )
    .await;

    if let Ok(x) = tags {
        let response = warp::reply::with_status(
            warp::reply::json(&x),
            StatusCode::OK,
        );
        Ok(response)
    } else {
        Err(ReplyError::Unauthorized.into())
    }
}
