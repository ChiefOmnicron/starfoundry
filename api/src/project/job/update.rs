use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectJobUuid, ProjectService, ProjectUuid, UpdateJob};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized, UnsupportedMediaType};
use crate::project::ProjectUuidPath;
use crate::project::job::ProjectJobUuidPath;

/// /projects/{projectUuid}/jobs/{projectJobUuid}
/// 
/// Updates the given job
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "project_job_update",
    path = "/projects/{projectUuid}/jobs/{projectJobUuid}",
    tag = "projects",
    params(
        ProjectUuidPath,
        ProjectJobUuidPath,
    ),
    request_body(
        content = UpdateJob,
        content_type = "application/json",
        description = "Update jobs",
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
        ("jwt" = ["project:read"])
    ),
)]
pub async fn update(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    job_uuid:     ProjectJobUuid,
    update:       UpdateJob,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.update_job(
        &pool,
        identity.character_id(),
        job_uuid,
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

