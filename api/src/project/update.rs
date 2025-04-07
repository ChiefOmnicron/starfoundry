use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectService, ProjectUuid, UpdateProject};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized, UnsupportedMediaType};
use crate::project::ProjectUuidPath;

/// /projects/{projectUuid}
/// 
/// Updates a project witht he given information
/// 
/// Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "project_update",
    path = "/projects/{projectUuid}",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    request_body(
        content = UpdateProject,
        description = "Update for the project",
    ),
    responses(
        NoContent,
        BadRequest,
        Unauthorized,
        Forbidden,
        UnsupportedMediaType,
        InternalServerError,
    ),
)]
pub async fn update(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    update:       UpdateProject,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.update(
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

