use sqlx::PgPool;
use starfoundry_libs_appraisal::internal::Appraisal;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::api_docs::{BadRequest, InternalServerError, NotFound};
use crate::ReplyError;

/// /api/v1/appraisal/:code
/// 
/// Fetches the appraisal with the given code
/// 
#[utoipa::path(
    get,
    operation_id = "appraisal_fetch",
    path = "/api/v1/appraisal/{code}",
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
    pool: PgPool,
    code: String,
) -> Result<impl Reply, Rejection> {
    match starfoundry_libs_appraisal::internal::fetch(
        &pool,
        code
    ).await {
        Ok(Some(x))  => Ok(warp::reply::json(&x)),
        Ok(None)  => Err(ReplyError::NotFound.into()),
        Err(e) => {
            tracing::error!("{}", e);
            Err(ReplyError::Internal.into())
        },
    }
}
