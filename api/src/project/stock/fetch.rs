use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectService, ProjectUuid, StockGroup};
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized};
use crate::project::ProjectUuidPath;

/// /projects/{projectUuid}/stocks
/// 
/// Fetches the stocks for the given project_uuid and returns them grouped
/// together based on their category_id or group_id.
/// 
/// Order:
/// - Compressed Minerals
/// - Minerals
/// - Moon Materials
/// - Compressed Gas
/// - Gas
/// - Fuel Blocks
/// - Intermediate Composite
/// - Composite
/// - Hybrid Polymers
/// - PI Tier 1
/// - PI Tier 2
/// - PI Tier 3
/// - PI Tier 4
/// - Commodities
/// - Construction Components
/// - Salvage
/// - Modules
/// - Charges
/// - Booster
/// - Ice
/// - Biochemical
/// - Abyssal Materials
/// - Ungrouped
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    operation_id = "project_stock_fetch",
    path = "/projects/{projectUuid}/stocks",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        (
            body = StockGroup,
            content_type = "application/json",
            description = "Stocks defined in the project, sorted by their market group",
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
pub async fn fetch(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.fetch_stock(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => Ok(warp::reply::json(&x.into_group())),
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
