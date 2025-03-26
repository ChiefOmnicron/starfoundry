use sqlx::PgPool;
use starfoundry_libs_structures::{StructureService, StructureUuid, UpdateStructure};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized, UnsupportedMediaType};
use super::StructureUuidPath;

#[utoipa::path(
    put,
    operation_id = "structures_update",
    path = "/api/v1/structures/{structureUuid}",
    tag = "structures",
    params(
        StructureUuidPath,
    ),
    request_body(
        content = UpdateStructure,
        description = "New information about the structure",
        content_type = "application/json"
    ),
    responses(
        (
            description = "The structure was updated",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        Forbidden,
        UnsupportedMediaType,
        InternalServerError,
    ),
)]
pub async fn update(
    pool:           PgPool,
    identity:       Identity,
    structure_uuid: StructureUuid,
    structure:      UpdateStructure,
) -> Result<impl Reply, Rejection> {
    let structure_wrapper = StructureService::new(structure_uuid);

    match structure_wrapper.update(
        &pool,
        identity.character_id(),
        structure,
    ).await {
        Ok(x) => Ok(
            warp::reply::with_status(
                warp::reply::json(&x),
                warp::http::StatusCode::OK,
            )
        ),
        Err(starfoundry_libs_structures::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_structures::Error::StructureNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error updating structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
