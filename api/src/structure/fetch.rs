use sqlx::PgPool;
use starfoundry_libs_structures::{Structure, StructureService, StructureUuid};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized};

#[utoipa::path(
    get,
    operation_id = "structures_fetch",
    path = "/api/v1/structures/{structureId}",
    tag = "structures",
    params(
        (
            "structureId" = StructureUuid,
            description = "UUID of the structure to fetch",
        ),
    ),
    responses(
        (
            body = Structure,
            content_type = "application/json",
            description = "Information about the requested structure",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn fetch(
    pool:           PgPool,
    identity:       Identity,
    structure_uuid: StructureUuid,
) -> Result<impl Reply, Rejection> {
    let structure_service = StructureService::new(structure_uuid);

    match structure_service.fetch(
        &pool,
        identity.character_id(),
    ).await {
        Ok(Some(x)) => Ok(warp::reply::json(&x)),
        Ok(None)    => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_structures::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_structures::Error::StructureNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error fetching structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
