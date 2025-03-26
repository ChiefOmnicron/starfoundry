use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectService, ProjectUuid, UpdateMisc};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized, UnsupportedMediaType};
use crate::project::ProjectUuidPath;
use crate::project::misc::ProjectMiscUuidPath;

/// /api/v1/projects/{projectUuid}/misc/{projectMiscUuid}
/// 
/// Updates the given misc entry
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "project_job_update",
    path = "/api/v1/projects/{projectUuid}/misc/{projectMiscUuid}",
    tag = "projects",
    params(
        ProjectUuidPath,
        ProjectMiscUuidPath,
    ),
    request_body(
        content = UpdateMisc,
        description = "Update misc",
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
    update:       UpdateMisc,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.update_misc(
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

