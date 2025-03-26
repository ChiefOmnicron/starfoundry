use sqlx::PgPool;
use starfoundry_libs_projects::{ProjectService, ProjectUuid, UpdateStockPrice};
use warp::{Reply, Rejection};
use warp::http::StatusCode;

use crate::{ReplyError, Identity};
use crate::api_docs::{BadRequest, Forbidden, InternalServerError, NoContent, Unauthorized, UnsupportedMediaType};
use crate::project::ProjectUuidPath;

/// /api/v1/projects/{projectUuid}/stocks/prices
/// 
/// Updates the prices of the excess items, either using the internal appraisal
/// tool or by using janice.
/// For janice to work an API key is required and the feature flag needs to be
/// enabled
/// 
/// ## Security
/// - authenticated
/// - project:write
/// 
#[utoipa::path(
    put,
    operation_id = "project_stock_update_price",
    path = "/api/v1/projects/{projectUuid}/stocks/prices",
    tag = "projects",
    params(
        ProjectUuidPath,
    ),
    request_body(
        content = UpdateStockPrice,
        content_type = "application/json",
        description = "Appraisal that should be used to update the price",
    ),
    responses(
        NoContent,
        BadRequest,
        Unauthorized,
        Forbidden,
        UnsupportedMediaType,
        InternalServerError,
    ),
    security (
        ("jwt" = ["project:write"])
    ),
)]
pub async fn update_price(
    pool:         PgPool,
    identity:     Identity,
    project_uuid: ProjectUuid,
    update:       UpdateStockPrice,
) -> Result<impl Reply, Rejection> {
    let project = ProjectService::new(project_uuid);

    match project.update_stock_price(
        &pool,
        identity.character_id(),
        update,
    ).await {
        Ok(x) => {
            Ok(
                warp::reply::with_status(
                    warp::reply::json(&x),
                    StatusCode::NO_CONTENT,
                )
            )
        },
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
