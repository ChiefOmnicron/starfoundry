use sqlx::PgPool;
use starfoundry_libs_projects::{CheckResources, ProjectService, Stock};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized, UnsupportedMediaType};

/// /api/v1/projects/check
///
/// Checks if there are enough ressources to build the selected items, based on
/// the given materials
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    operation_id = "project_check_resources",
    path = "/api/v1/projects/check",
    tag = "projects",
    request_body = CheckResources,
    responses(
        (
            body = Vec<Stock>,
            content_type = "application/json",
            description = "List of missing materials",
            status = OK,
        ),
        NoContent,
        BadRequest,
        Unauthorized,
        Forbidden,
        UnsupportedMediaType,
        InternalServerError,
    ),
    security (
        ("jwt" = [])
    ),
)]
pub async fn check_resources(
    pool:            PgPool,
    identity:        Identity,
    check_resources: CheckResources,
) -> Result<impl Reply, Rejection> {
    match ProjectService::check_resources(
        &pool,
        identity.character_id(),
        check_resources,
    ).await {
        Ok(x) => {
            if x.is_empty() {
                let empty: Vec<Stock> = Vec::new();

                Ok(warp::reply::with_status(
                    warp::reply::json(&empty),
                    StatusCode::NO_CONTENT,
                ))
            } else {
                Ok(warp::reply::with_status(
                    warp::reply::json(&x),
                    StatusCode::OK,
                ))
            }
        },
        Err(starfoundry_libs_projects::Error::StructureNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
