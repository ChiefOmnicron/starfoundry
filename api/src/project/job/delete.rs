use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectJobUuid, ProjectService, ProjectUuid};
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::project::job::ProjectJobUuidPath;
use crate::project::ProjectUuidPath;

#[utoipa::path(
    delete,
    operation_id = "project_job_delete",
    path = "/api/v1/projects/{projectUuid}/jobs/{projectJobUuid}",
    tag = "projects",
    params(
        ProjectUuidPath,
        ProjectJobUuidPath,
    ),
    responses(
        (
            description = "Job was deleted",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn delete(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    job_uuid:     ProjectJobUuid,
) -> Result<impl Reply, Rejection> {
    let service = ProjectService::new(project_uuid);

    match service.delete_job(
        &pool,
        identity.character_id(),
        job_uuid,
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
