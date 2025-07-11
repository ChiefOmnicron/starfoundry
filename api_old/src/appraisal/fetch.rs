use sqlx::PgPool;
use starfoundry_libs_appraisal::internal::Appraisal;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{BadRequest, InternalServerError, NotFound};
use crate::metric::{RequestStatus, WithMetric};
use crate::ReplyError;

/// /appraisals/:code
/// 
/// Fetches the appraisal with the given code
/// 
#[utoipa::path(
    get,
    operation_id = "appraisal_fetch",
    path = "/appraisals/{code}",
    tag = "appraisal",
    params(
        ("code" = String, Path),
    ),
    responses(
        (
            body = Appraisal,
            content_type = "application/json",
            description = "ID of the new project",
            status = OK,
        ),
        BadRequest,
        NotFound,
        InternalServerError,
    ),
)]
pub async fn fetch(
    pool:   PgPool,
    metric: WithMetric,
    code:   String,
) -> Result<impl Reply, Rejection> {
    match starfoundry_libs_appraisal::internal::fetch(
        &pool,
        code
    ).await {
        Ok(Some(x))  => {
            metric.inc_appraisal_fetch_count(RequestStatus::Ok);
            Ok(warp::reply::json(&x))
        },
        Ok(None)  => {
            metric.inc_appraisal_fetch_count(RequestStatus::NotFound);
            Err(ReplyError::NotFound.into())
        },
        Err(e) => {
            tracing::error!("{}", e);
            metric.inc_appraisal_fetch_count(RequestStatus::Error);
            Err(ReplyError::Internal.into())
        },
    }
}
