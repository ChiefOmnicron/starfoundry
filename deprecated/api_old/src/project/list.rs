use sqlx::PgPool;
use starfoundry_lib_projects::{ProjectFilter, ProjectService, ProjectUuid};
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::error::ReplyError;
use crate::Identity;

/// /projects
/// 
/// Fetches a list of projects that match the given filter
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_list",
    path = "/projects",
    tag = "projects",
    params(ProjectFilter),
    responses(
        (
            body = Vec<ProjectUuid>,
            description = "List of projects the character has access to",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn list(
    pool:     PgPool,
    identity: Identity,
    filter:   ProjectFilter,
) -> Result<impl Reply, Rejection> {
    match ProjectService::list(
        &pool,
        identity.character_id(),
        filter,
    ).await {
        Ok(x) => {
            Ok(warp::reply::with_status(
                warp::reply::json(&x),
                StatusCode::OK,
            ))
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
