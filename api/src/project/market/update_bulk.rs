use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectService, ProjectUuid, UpdateMarket};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized, UnsupportedMediaType};
use crate::project::ProjectUuidPath;

/// /api/v1/projects/{projectUuid}/market/bulk
/// 
/// Bulk updates market entries by their type_id
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "project_market_update",
    path = "/api/v1/projects/{projectUuid}/market/bulk",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    request_body(
        content = Vec<UpdateMarket>,
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
pub async fn update_bulk(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    updates:      Vec<UpdateMarket>,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.update_bulk_market(
        &pool,
        identity.character_id(),
        updates,
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

