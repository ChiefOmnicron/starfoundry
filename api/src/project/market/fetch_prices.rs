use sqlx::PgPool;
use starfoundry_libs_projects::{MarketRecommendation, ProjectService, ProjectUuid};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized};
use crate::project::ProjectUuidPath;

/// /api/v1/projects/{projectUuid}/market/prices
/// 
/// Fetches the market prices for all materials required
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_market_fetch_prices",
    path = "/api/v1/projects/{projectUuid}/market/prices",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        (
            body = Vec<MarketRecommendation>,
            content_type = "application/json",
            description = "Recommended material buy locations",
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
pub async fn fetch_prices(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.fetch_market_prices(
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
