use sqlx::PgPool;
use starfoundry_libs_projects::{AddMarket, ProjectUuid, ProjectService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};
use crate::project::ProjectUuidPath;
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized};

/// /projects/{projectUuid}/market
/// 
/// Adds an additional market entry
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    post,
    operation_id = "project_add_market_entries",
    path = "/projects/{projectUuid}/market",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    request_body(
        content = AddMarket,
        content_type = "application/json",
        description = "New entries to add",
    ),
    responses(
        (
            description = "Market entries added",
            status = NO_CONTENT,
        ),
        BadRequest,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security (
        ("jwt" = ["project:write"])
    ),
)]
pub async fn add(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    entry:        AddMarket,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.add_market(
        &pool,
        identity.character_id(),
        entry,
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
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
