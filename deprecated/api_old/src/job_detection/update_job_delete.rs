use sqlx::PgPool;
use starfoundry_lib_projects::{JobDetectionService, UpdateJobDetectionDelete};
use starfoundry_lib_types::JobId;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{Forbidden, InternalServerError, NoContent, Unauthorized};
use crate::job_detection::JobIdPath;

/// /job-detection/{jobId}/delete
/// 
/// Deletes a job from a project, and optionaly also ignores it in future
/// job detections.
/// 
/// If the job is already assigned, the identity MUST have write permissions
/// to that project
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "job_detection_update_job_delete",
    path = "/job-detection/{jobId}/delete",
    tag = "project-job-detection",
    params(
        JobIdPath,
    ),
    request_body(
        content = UpdateJobDetectionDelete,
    ),
    responses(
        NoContent,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn update_job_delete(
    pool:      PgPool,
    identity: Identity,
    job_id:   JobId,
    update:   UpdateJobDetectionDelete,
) -> Result<impl Reply, Rejection> {
    match JobDetectionService::update_job_delete(
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
        Err(starfoundry_lib_projects::Error::ProjectNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(starfoundry_lib_projects::Error::Forbidden(_, _)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
