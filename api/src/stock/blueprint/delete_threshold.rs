use sqlx::PgPool;
use warp::reject::Rejection;
use warp::reply::Reply;
use warp::http::StatusCode;

use crate::{Identity, ReplyError};
use super::{BlueprintStockError, BlueprintStockUuid};

pub async fn delete_threshold(
    pool:         &PgPool,
    threshold_id: BlueprintStockUuid,
) -> Result<(), BlueprintStockError> {
    let result = sqlx::query!("
            DELETE FROM stock_blueprint_threshold
            WHERE id = $1
        ",
            *threshold_id,
        )
        .execute(pool)
        .await
        .map_err(|e| BlueprintStockError::DeleteThreshold(e, threshold_id))?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(BlueprintStockError::ThresholdNotFound(threshold_id))
    }
}

/// /stocks/blueprints/{blueprintStockId}/thresholds/{thresholdId}
/// 
#[utoipa::path(
    delete,
    operation_id = "stocks_blueprints_thresholds_delete",
    path = "/stocks/blueprints/{blueprintStockId}/thresholds/{thresholdId}",
    tag = "blueprint-stocks",
    params(
        (
            "blueprintStockId" = BlueprintStockUuid,
            description = "UUID of the blueprint stock",
        ),
        (
            "thresholdId" = BlueprintStockUuid,
            description = "UUID of the threshold to delete",
        ),
    ),
    responses(
        (
            description = "The threshold was deleted",
            status = NO_CONTENT,
        ),
        (
            description = "Invalid parameter",
            status = BAD_REQUEST,
        ),
        (
            description = "Not Found",
            status = NOT_FOUND,
        ),
        (
            description = "The requester is not authenticated",
            status = UNAUTHORIZED,
        ),
        (
            description = "Unknown error",
            status = INTERNAL_SERVER_ERROR,
        ),
    ),
)]
pub async fn delete_threshold_api(
    pool:         PgPool,
    _:            Identity,
    _:            BlueprintStockUuid,
    threshold_id: BlueprintStockUuid,
) -> Result<impl Reply, Rejection> {
    match delete_threshold(
        &pool,
        threshold_id,
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(BlueprintStockError::ThresholdNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error deleting bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
