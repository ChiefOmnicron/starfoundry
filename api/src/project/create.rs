use sqlx::PgPool;
use starfoundry_libs_projects::{CreateProject, ProjectService, ProjectUuid};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnsupportedMediaType};

/// /projects
/// 
/// Creates a new project and returns it's id
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    operation_id = "projects_create",
    path = "/projects",
    tag = "projects",
    request_body(
        content = CreateProject,
        description = "Information about the project",
        content_type = "application/json"
    ),
    responses(
        (
            body = ProjectUuid,
            content_type = "application/json",
            description = "ID of the new project",
            status = CREATED,
        ),
        BadRequest,
        Unauthorized,
        UnsupportedMediaType,
        InternalServerError,
    ),
    security (
        ("jwt" = [])
    ),
)]
pub async fn create(
    pool:      PgPool,
    identity:  Identity,
    structure: CreateProject,
) -> Result<impl Reply, Rejection> {
    match ProjectService::create(
        &pool,
        identity.character_id(),
        structure,
    ).await {
        Ok(x) => Ok(
            warp::reply::with_status(
                warp::reply::json(&x),
                warp::http::StatusCode::CREATED,
            ),
        ),
        Err(e) => {
            tracing::error!("Unexpected error creating structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
