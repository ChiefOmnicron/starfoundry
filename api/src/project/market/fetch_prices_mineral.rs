use sqlx::PgPool;
use starfoundry_libs_projects::{MarketRecommendation, ProjectService, ProjectUuid};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized};
use crate::project::ProjectUuidPath;

/// /projects/{projectUuid}/market/prices/minerals
/// 
/// Fetches the market prices for minerals
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_market_fetch_prices_minerals",
    path = "/projects/{projectUuid}/market/prices/minerals",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        (
            body = Vec<MarketRecommendation>,
            content_type = "application/json",
            description = "Recommended minerals",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security (
        ("jwt" = ["project:read"])
    ),
)]
pub async fn fetch_prices_minerals(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.fetch_market_prices_minerals(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
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
