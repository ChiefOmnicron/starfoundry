use sqlx::PgPool;
use starfoundry_libs_structures::{StructureService, StructureUuid};
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized};

/// /structures/{structureId}
/// 
#[utoipa::path(
    delete,
    operation_id = "structures_delete",
    path = "/structures/{structureId}",
    tag = "structures",
    params(
        (
            "structureId" = StructureUuid,
            description = "UUID of the structure to update",
        ),
    ),
    responses(
        NoContent,
        BadRequest,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn delete(
    pool:           PgPool,
    identity:       Identity,
    structure_uuid: StructureUuid,
) -> Result<impl Reply, Rejection> {
    let structure_service = StructureService::new(structure_uuid);

    match structure_service.delete(
        &pool,
        identity.character_id(),
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(starfoundry_libs_structures::Error::StructureNotFound(structure_id)) => {
            tracing::warn!("structure id not found {structure_id}");
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error fetching structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
