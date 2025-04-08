use sqlx::PgPool;
use starfoundry_libs_appraisal::internal::{Appraisal, ReprocessingOptions};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{BadRequest, InternalServerError, NotFound};
use crate::metric::{RequestStatus, WithMetric};
use crate::ReplyError;

/// /appraisal/:code/reprocessing
/// 
/// Compresses the materials from the given appraisal
/// 
#[utoipa::path(
    put,
    operation_id = "appraisal_reprocessing",
    path = "/appraisal/{code}/reprocessing",
    tag = "appraisal",
    params(
        ("code" = String, Path),
    ),
    request_body(
        content = ReprocessingOptions,
        description = "Options that are considered during reprocessing calculation",
        content_type = "application/json"
    ),
    responses(
        (
            body = Appraisal,
            content_type = "application/json",
            description = "Appraisal for the reprocessed materials",
            status = OK,
        ),
        BadRequest,
        NotFound,
        InternalServerError,
    ),
)]
pub async fn reprocessing(
    pool:    PgPool,
    metric:  WithMetric,
    code:    String,
    options: ReprocessingOptions,
) -> Result<impl Reply, Rejection> {
    match starfoundry_libs_appraisal::internal::reprocessing(
        &pool,
        code,
        options,
    ).await {
        Ok(Some(x))  => {
            metric.inc_appraisal_reprocessing_count(RequestStatus::Ok);
            Ok(warp::reply::json(&x))
        },
        Ok(None)  => {
            metric.inc_appraisal_reprocessing_count(RequestStatus::NotFound);
            Err(ReplyError::NotFound.into())
        },
        Err(e) => {
            tracing::error!("{}", e);
            metric.inc_appraisal_reprocessing_count(RequestStatus::Error);
            Err(ReplyError::Internal.into())
        },
    }
}
