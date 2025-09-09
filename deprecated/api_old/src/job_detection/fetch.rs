use sqlx::PgPool;
use starfoundry_lib_projects::{JobDetection, JobDetectionService};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};

/// /job-detection
/// 
/// Fetches all non delivered job detection logs
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    get,
    operation_id = "job_detection_fetch",
    path = "/job-detection",
    tag = "project-job-detection",
    responses(
        (
            body = Vec<JobDetection>,
            content_type = "application/json",
            description = "List of all non delivered detected jobs",
            status = OK,
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
)]
pub async fn fetch(
    pool:     PgPool,
    identity: Identity,
) -> Result<impl Reply, Rejection> {
    match JobDetectionService::fetch(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
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
