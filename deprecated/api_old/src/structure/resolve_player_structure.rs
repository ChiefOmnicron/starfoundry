use sqlx::PgPool;
use starfoundry_lib_structures::{ResolvedStructure, StructureService};
use starfoundry_lib_types::StructureId;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError};

/// /structures/{structureId}/resolve
/// 
#[utoipa::path(
    get,
    operation_id = "structures_resolve_player",
    path = "/structures/{structureId}/resolve",
    tag = "Structures",
    params(
        (
            "structureId" = i64,
            description = "In-game structure ID",
        ),
    ),
    responses(
        (
            description = "Information about the structure",
            body = ResolvedStructure,
            status = OK,
        ),
        BadRequest,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn resolve_player_structure(
    pool:         PgPool,
    identity:     Identity,
    structure_id: StructureId,
) -> Result<impl Reply, Rejection> {
    match StructureService::resolve_player_structure(
        &pool,
        identity.api_client().await?,
        structure_id,
    ).await {
        Ok(x) => Ok(
            warp::reply::with_status(
                warp::reply::json(&x),
                warp::http::StatusCode::OK,
            )
        ),
        Err(starfoundry_lib_structures::Error::InvalidStructureId(_)) => {
            Err(ReplyError::BadRequest.into())
        }
        Err(starfoundry_lib_structures::Error::FetchPlayerStructureFromEve(e, _)) => {
            tracing::error!("{}", e);
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("{}", e);
            Err(ReplyError::Internal.into())
        },
    }
}
