use sqlx::PgPool;
use starfoundry_libs_projects::{JobDetectionService, UpdateJobDetectionAdd};
use starfoundry_libs_types::JobId;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{Forbidden, InternalServerError, NoContent, Unauthorized};
use crate::job_detection::JobIdPath;

/// /job-detection/{jobId}/add
/// 
/// Adds the given entry to a project.
/// 
/// If the job is already assigned, the identity MUST to have write permissions
/// on both projects.
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "job_detection_update_job_add",
    path = "/job-detection/{jobId}/add",
    tag = "project-job-detection",
    params(
        JobIdPath,
    ),
    request_body(
        content = UpdateJobDetectionAdd,
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
pub async fn update_job_add(
    pool:     PgPool,
    identity: Identity,
    job_id:   JobId,
    update:   UpdateJobDetectionAdd,
) -> Result<impl Reply, Rejection> {
    match JobDetectionService::update_job_add(
        &pool,
        identity.character_id(),
        job_id,
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
        Err(starfoundry_libs_projects::Error::ProjectNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_libs_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
