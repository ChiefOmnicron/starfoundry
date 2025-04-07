use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectMarketUuid, ProjectService, ProjectUuid, UpdateMarket};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized, UnsupportedMediaType};
use crate::project::ProjectUuidPath;
use crate::project::market::ProjectMarketUuidPath;

/// /projects/{projectUuid}/market/{projectMarketUuid}
/// 
/// Updates a market entry
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "project_market_update",
    path = "/projects/{projectUuid}/market/{projectMarketUuid}",
    tag = "projects",
    params(
        ProjectUuidPath,
        ProjectMarketUuidPath,
    ),
    request_body(
        content = UpdateMarket,
        description = "Update market",
    ),
    responses(
        NoContent,
        BadRequest,
        Unauthorized,
        Forbidden,
        UnsupportedMediaType,
        InternalServerError,
    ),
    security (
        ("jwt" = ["project:write"])
    ),
)]
pub async fn update(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    market_uuid:  ProjectMarketUuid,
    update:       UpdateMarket,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.update_market(
        &pool,
        identity.character_id(),
        market_uuid,
        update,
    ).await {
        Ok(x) => {
            Ok(
                warp::reply::with_status(
                    warp::reply::json(&x),
                    StatusCode::NO_CONTENT,
                )
            )
        },
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::ProjectNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

