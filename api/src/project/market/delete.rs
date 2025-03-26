use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectMarketUuid, ProjectService, ProjectUuid};
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{Forbidden, InternalServerError, NoContent, Unauthorized};
use crate::project::ProjectUuidPath;
use crate::project::market::ProjectMarketUuidPath;

/// /api/v1/projects/{projectUuid}/market/{projectMarketUuid}
/// 
/// Removes a market entry
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    delete,
    operation_id = "project_market_delete",
    path = "/api/v1/projects/{projectUuid}/market/{projectMarketUuid}",
    tag = "projects",
    params(
        ProjectUuidPath,
        ProjectMarketUuidPath,
    ),
    responses(
        NoContent,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security (
        ("jwt" = ["project:write"])
    ),
)]
pub async fn delete(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    market_uuid:  ProjectMarketUuid,
) -> Result<impl Reply, Rejection> {
    let service = ProjectService::new(project_uuid);

    match service.delete_market(
        &pool,
        identity.character_id(),
        market_uuid,
    ).await {
        Ok(x) => {
            let response = warp::reply::with_status(
                warp::reply::json(&x),
                StatusCode::NO_CONTENT,
            );
            Ok(response)
        },
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::ProjectGroupNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
