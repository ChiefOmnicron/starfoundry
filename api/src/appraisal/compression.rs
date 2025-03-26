use sqlx::PgPool;
use starfoundry_libs_appraisal::internal::{CompressionOptions, CompressionResult};
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{BadRequestWithPayload, InternalServerError, NotFound};
use crate::{BadRequestPayload, ReplyError};

/// /api/v1/appraisal/:code/compression
/// 
/// Compresses the materials from the given appraisal
/// 
#[utoipa::path(
    put,
    operation_id = "appraisal_compression",
    path = "/api/v1/appraisal/{code}/compression",
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
    code:    String,
    options: CompressionOptions,
) -> Result<impl Reply, Rejection> {
    match starfoundry_libs_appraisal::internal::compression(
        &pool,
        code,
        options,
    ).await {
        Ok(Some(x))  => Ok(warp::reply::json(&x)),
        Ok(None)  => Err(ReplyError::NotFound.into()),
        Err(starfoundry_libs_appraisal::Error::NoSolution) => {
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
            Err(ReplyError::Internal.into())
        },
    }
}
