use sqlx::PgPool;
use starfoundry_libs_projects::{AddMisc, ProjectUuid, ProjectService};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{Identity, ReplyError};
use crate::project::ProjectUuidPath;
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, Unauthorized};

/// /projects/{projectUuid}/misc
/// 
/// Adds a new misc entry
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    post,
    operation_id = "project_add_misc_entries",
    path = "/projects/{projectUuid}/misc",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    request_body(
        content = AddMisc,
        description = "New entries to add",
        content_type = "application/json",
    ),
    responses(
        (
            description = "Misc entries added",
            status = CREATED,
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
    entry:        AddMisc,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.add_misc(
        &pool,
        identity.character_id(),
        entry,
    ).await {
        Ok(x) => {
            Ok(
                warp::reply::with_status(
                    warp::reply::json(&x),
                    StatusCode::CREATED,
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
