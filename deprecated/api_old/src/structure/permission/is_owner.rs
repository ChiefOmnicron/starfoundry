use sqlx::PgPool;
use starfoundry_lib_structures::{StructureUuid, StructureService};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::structure::StructureUuidPath;
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};

/// /structures/{structureUuid}/permissions/is-owner
/// 
/// Checks if the character is the owner of the structure.
/// Any positive HTTP-Statuscode 2xx should be considered as allowed
/// Any non positive HTTP-Statuscode 4xx, 5xx should be considered as forbidden
/// 
#[utoipa::path(
    get,
    operation_id = "structure_permission_is_owner",
    path = "/structures/{structureUuid}/permissions/is-owner",
    tag = "structures",
    params(
        StructureUuidPath,
    ),
    responses(
        (
            description = "The requester is owner",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn is_owner(
    pool:           PgPool,
    identity:       Identity,
    structure_uuid: StructureUuid,
) -> Result<impl Reply, Rejection> {
    let structure_service = StructureService::new(structure_uuid);

    match structure_service.assert_owner(
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
