use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectService, ProjectUuid, UpdateMineral};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized, UnsupportedMediaType};
use crate::project::ProjectUuidPath;

/// /projects/{projectUuid}/market/minerals
/// 
/// Replaces the raw minerals with the given compressed ores.
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "project_market_update_minerals",
    path = "/projects/{projectUuid}/market/minerals",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    request_body(
        content = Vec<UpdateMineral>,
        description = "Update market minerals",
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
pub async fn update_minerals(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    update:       Vec<UpdateMineral>,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.update_market_minerals(
        &pool,
        identity.character_id(),
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

