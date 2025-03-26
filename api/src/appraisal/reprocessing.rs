use sqlx::PgPool;
use starfoundry_libs_appraisal::internal::{Appraisal, ReprocessingOptions};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{BadRequest, InternalServerError, NotFound};
use crate::ReplyError;

/// /api/v1/appraisal/:code/reprocessing
/// 
/// Compresses the materials from the given appraisal
/// 
#[utoipa::path(
    put,
    operation_id = "appraisal_reprocessing",
    path = "/api/v1/appraisal/{code}/reprocessing",
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
    code:    String,
    options: ReprocessingOptions,
) -> Result<impl Reply, Rejection> {
    match starfoundry_libs_appraisal::internal::reprocessing(
        &pool,
        code,
        options,
    ).await {
        Ok(Some(x))  => Ok(warp::reply::json(&x)),
        Ok(None)  => Err(ReplyError::NotFound.into()),
        Err(e) => {
            tracing::error!("{}", e);
            Err(ReplyError::Internal.into())
        },
    }
}
