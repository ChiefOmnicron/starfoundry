use sqlx::PgPool;
use starfoundry_lib_projects::ProjectService;
use starfoundry_lib_structures::StructureUuid;
use warp::{Reply, Rejection};

use crate::ReplyError;
use crate::api_docs::{Forbidden, InternalServerError};
use crate::structure::StructureUuidPath;

/// /projects/market/{structureUuid}/last-fetch
/// 
/// Returns the date, the market data was last fetched
/// 
#[utoipa::path(
    get,
    operation_id = "project_market_last_fetch",
    path = "/projects/market/{structureUuid}/last-fetch",
    tag = "projects",
    params(
        StructureUuidPath,
    ),
    responses(
        (
            body = String,
            description = "Last datetime the market was fetched. Format: RFC3339",
            status = OK,
        ),
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn last_fetch(
    pool:           PgPool,
    structure_uuid: StructureUuid,
) -> Result<impl Reply, Rejection> {
    match ProjectService::last_market_fetch(
        &pool,
        structure_uuid,
    ).await {
        Ok(x) => {
            let date_time = x.map(|x| {
                x.format("%Y-%m-%dT%TZ").to_string()
            });

            Ok(warp::reply::json(&date_time))
        },
        Err(starfoundry_lib_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_lib_projects::Error::ProjectNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
