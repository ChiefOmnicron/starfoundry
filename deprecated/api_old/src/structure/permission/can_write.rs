use sqlx::PgPool;
use starfoundry_lib_structures::{StructureService, StructureUuid};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::structure::StructureUuidPath;

/// /structures/{structureUuid}/permissions/can-write
/// 
/// Checks if the character is allowed to write the structure.
/// Any positive HTTP-Statuscode 2xx should be considered as allowed
/// Any non positive HTTP-Statuscode 4xx, 5xx should be considered as forbidden
/// 
#[utoipa::path(
    get,
    operation_id = "structrues_permission_can_write",
    path = "/structures/{structureUuid}/permissions/can-write",
    tag = "structures",
    params(
        StructureUuidPath,
    ),
    responses(
        (
            description = "The requester has write access",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn can_write(
    pool:           PgPool,
    identity:       Identity,
    structure_uuid: StructureUuid,
) -> Result<impl Reply, Rejection> {
    let structure_service = StructureService::new(structure_uuid);

    match structure_service.assert_write_access(
        &pool,
        identity.character_id(),
    ).await {
        Ok(_) => Ok(warp::reply::json(&())),
        Err(starfoundry_lib_structures::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_lib_structures::Error::StructureNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
