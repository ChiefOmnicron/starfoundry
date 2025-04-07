use sqlx::PgPool;
use starfoundry_libs_projects::{CostEstimateConfiguration, CostEstimateResponse, ProjectService};
use warp::{Reply, Rejection};

use crate::{Identity, ReplyError};
use crate::api_docs::{BadRequest, InternalServerError, Unauthorized, UnsupportedMediaType};

/// /projects/cost-estimate
/// 
/// creates an estimate of the cost to build the given products
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    operation_id = "project_cost_estimate",
    path = "/projects/cost-estimate",
    tag = "projects",
    request_body = CostEstimateConfiguration,
    responses(
        (
            body = CostEstimateResponse,
            content_type = "application/json",
            description = "List of missing materials",
            status = OK,
        ),
        BadRequest,
        Unauthorized,
        UnsupportedMediaType,
        InternalServerError,
    ),
    security (
        ("jwt" = []),
    ),
)]
pub async fn cost_estimate(
    pool:     PgPool,
    identity: Identity,
    config:   CostEstimateConfiguration,
) -> Result<impl Reply, Rejection> {
    match ProjectService::cost_estimate(
        &pool,
        identity.character_id(),
        config,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(e) => {
            tracing::error!("Unexpected error, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
