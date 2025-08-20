use sqlx::PgPool;
use starfoundry_lib_structures::{Structure, StructureListFilter, StructureService};
use warp::{Reply, Rejection};

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::{ReplyError, Identity};

/// /structures
/// 
/// Alternative route: `/v1/structures`
/// 
/// ---
/// 
/// Lists all structures the user has access to.
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    operation_id = "structures_list",
    path = "/structures",
    tag = "structures",
    params(StructureListFilter),
    responses(
        (
            body = Vec<Structure>,
            content_type = "application/json",
            description = "List of all matching structures the user has access to",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn list(
    pool:     PgPool,
    identity: Identity,
    filter:   StructureListFilter,
) -> Result<impl Reply, Rejection> {
    match StructureService::list(
        &pool,
        identity.character_id(),
        filter,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(starfoundry_lib_structures::Error::ListStructureIds(e, character_id, filter)) => {
            tracing::error!(
                "error listing structure ids from owner '{}', filter: {}, error: '{}'",
                character_id,
                filter,
                e,
            );
            Err(ReplyError::Internal.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error listing structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
