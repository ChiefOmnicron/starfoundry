use sqlx::PgPool;
use starfoundry_libs_structures::{StructureListFilter, StructureService};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};

#[utoipa::path(
    get,
    operation_id = "structures_list",
    path = "/api/v1/structures",
    tag = "structures",
    params(StructureListFilter),
    responses(
        (
            body = Vec<Uuid>,
            content_type = "application/json",
            description = "List of all matching structures the user has access to",
            status = OK,
        ),
        (
            description = "Invalid parameter",
            status = BAD_REQUEST,
        ),
        (
            description = "The requester is not authenticated",
            status = UNAUTHORIZED,
        ),
        (
            description = "Unknown error",
            status = INTERNAL_SERVER_ERROR,
        ),
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
        Err(starfoundry_libs_structures::Error::ListStructureIds(e, character_id, filter)) => {
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
