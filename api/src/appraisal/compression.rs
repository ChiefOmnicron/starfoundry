use sqlx::PgPool;
use starfoundry_libs_appraisal::internal::{CompressionOptions, CompressionResult};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{BadRequestWithPayload, InternalServerError, NotFound};
use crate::metric::{RequestStatus, WithMetric};
use crate::{BadRequestPayload, ReplyError};

/// /appraisal/:code/compression
/// 
/// Compresses the materials from the given appraisal
/// 
#[utoipa::path(
    put,
    operation_id = "appraisal_compression",
    path = "/appraisal/{code}/compression",
    tag = "appraisal",
    params(
        ("code" = String, Path),
    ),
    request_body(
        content = CompressionOptions,
        description = "Options that are considered during compression calculation",
        content_type = "application/json"
    ),
    responses(
        (
            body = CompressionResult,
            content_type = "application/json",
            description = "Appraisal for the compressed ores. Will not contain a code. Also includes an appraisal for the overage",
            status = OK,
        ),
        BadRequestWithPayload,
        NotFound,
        InternalServerError,
    ),
)]
pub async fn compression(
    pool:    PgPool,
    metric:  WithMetric,
    code:    String,
    options: CompressionOptions,
) -> Result<impl Reply, Rejection> {
    match starfoundry_libs_appraisal::internal::compression(
        &pool,
        code,
        options,
    ).await {
        Ok(Some(x))  => {
            metric.inc_appraisal_compression_count(RequestStatus::Ok);
            Ok(warp::reply::json(&x))
        },
        Ok(None)  => {
            metric.inc_appraisal_compression_count(RequestStatus::NotFound);
            Err(ReplyError::NotFound.into())
        },
        Err(starfoundry_libs_appraisal::Error::NoSolution) => {
            metric.inc_appraisal_compression_count(RequestStatus::NoSolution);
            Err(ReplyError::BadRequestWithPayload(
                    BadRequestPayload {
                        error: "NO_SOLUTION".into(),
                        description: "No solution found for compression request. Adjust the parameters.".into(),
                    }
                ).into()
            )
        },
        Err(e) => {
            tracing::error!("{}", e);
            metric.inc_appraisal_compression_count(RequestStatus::Error);
            Err(ReplyError::Internal.into())
        },
    }
}
