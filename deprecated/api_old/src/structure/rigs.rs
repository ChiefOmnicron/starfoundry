use sqlx::PgPool;
use starfoundry_lib_structures::{StructureRig, StructureService};
use starfoundry_lib_types::TypeId;
use warp::{Reply, Rejection};

use crate::ReplyError;
use crate::api_docs::{BadRequest, InternalServerError};

/// /structures/{structureTypeId}/rigs
/// 
#[utoipa::path(
    get,
    operation_id = "structures_rigs_list",
    path = "/structures/{structureTypeId}/rigs",
    tag = "Structures",
    params(
        (
            "structureTypeId" = TypeId,
            description = "TypeID of the structure",
            example = json!(35834)
        ),
    ),
    responses(
        (
            body = Vec<StructureRig>,
            content_type = "application/json",
            description = "List of all Rigs that are supported by the structure",
            status = OK,
        ),
        BadRequest,
        InternalServerError,
    ),
)]
pub async fn rig_by_structure_type_id(
    pool:         PgPool,
    structure_id: TypeId,
) -> Result<impl Reply, Rejection> {
    match StructureService::rig_by_structure_type_id(
        &pool,
        structure_id,
    ).await {
        Ok(x) => Ok(
            warp::reply::with_status(
                warp::reply::json(&x),
                warp::http::StatusCode::OK,
            )
        ),
        Err(e) => {
            tracing::error!("Unexpected error updating structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
