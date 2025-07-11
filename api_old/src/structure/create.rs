use sqlx::PgPool;
use starfoundry_libs_structures::{CreateStructure, StructureService, StructureUuid};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, InternalServerError};

/// /structures
/// 
#[utoipa::path(
    post,
    operation_id = "structures_create",
    path = "/structures",
    tag = "structures",
    request_body(
        content = CreateStructure,
        description = "Information about the structure",
        content_type = "application/json"
    ),
    responses(
        (
            body = StructureUuid,
            content_type = "application/json",
            description = "Information about the requested structure",
            status = CREATED,
        ),
        BadRequest,
        InternalServerError,
    ),
)]
pub async fn create(
    pool:      PgPool,
    identity:  Identity,
    structure: CreateStructure,
) -> Result<impl Reply, Rejection> {
    match StructureService::create(
        &pool,
        identity.character_id(),
        structure,
    ).await {
        Ok(x) => Ok(
            warp::reply::with_status(
                warp::reply::json(&x),
                warp::http::StatusCode::CREATED,
            )
        ),
        Err(e) => {
            tracing::error!("Unexpected error creating structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
