use sqlx::PgPool;
use starfoundry_libs_projects::{ExcessGroup, ProjectService, ProjectUuid};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use crate::project::ProjectUuidPath;
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized};

/// /api/v1/projects/{projectUuid}/excess
/// 
/// Fetches the stocks for the given project_uuid and returns them grouped,
/// this will return them grouped based on their category_id and group_id.
/// 
/// ## Order:
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
    operation_id = "project_excess_fetch",
    path = "/api/v1/projects/{projectUuid}/excess",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    responses(
        (
            body = ExcessGroup,
            content_type = "application/json",
            description = "Excess that is left after the project is finished",
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

    match project.fetch_excess(
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
