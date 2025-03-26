use sqlx::PgPool;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use super::{AddThreshold, BlueprintStockError, BlueprintStockUuid};

pub async fn update_threshold(
    pool:               &PgPool,
    blueprint_stock_id: BlueprintStockUuid,
    thresholds:         Vec<AddThreshold>,
) -> Result<(), BlueprintStockError> {
    sqlx::query!("
            DELETE FROM stock_blueprint_thresholds
            WHERE blueprint_stock_id = $1
        ",
            *blueprint_stock_id,
        )
        .execute(pool)
        .await
        .map_err(|e| BlueprintStockError::DeleteThreshold(e, blueprint_stock_id))?;

    let mut type_ids = Vec::new();
    let mut want = Vec::new();
    let mut critical = Vec::new();
    let mut min_runs = Vec::new();
    let mut min_me = Vec::new();
    let mut min_te = Vec::new();

    for threshold in thresholds {
        type_ids.push(*threshold.type_id);
        want.push(threshold.want);
        critical.push(threshold.critical);
        min_runs.push(threshold.min_runs);
        min_me.push(threshold.min_me);
        min_te.push(threshold.min_te);
    }

    sqlx::query!("
            INSERT INTO stock_blueprint_thresholds (
                blueprint_stock_id,
                type_id,
                want,
                critical,
                min_runs,
                min_me,
                min_te
            )
            SELECT $1, * FROM UNNEST(
                $2::INTEGER[],
                $3::INTEGER[],
                $4::INTEGER[],
                $5::INTEGER[],
                $6::INTEGER[],
                $7::INTEGER[]
            )
        ",
            *blueprint_stock_id,
            &type_ids,
            &want,
            &critical,
            &min_runs,
            &min_me,
            &min_te
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| BlueprintStockError::AddThreshold(e, blueprint_stock_id))
}

#[utoipa::path(
    put,
    operation_id = "stocks_blueprints_thresholds_update",
    path = "/api/v1/stocks/blueprints/{blueprintStockId}/thresholds",
    tag = "blueprint-stocks",
    params(
        (
            "blueprintStockId" = Vec<BlueprintStockUuid>,
            description = "UUID of the bpc stock to update",
        ),
    ),
    request_body = Vec<AddThreshold>,
    responses(
        (
            description = "Entry udpated",
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
            description = "Only application/json is supported",
            status = UNSUPPORTED_MEDIA_TYPE,
        ),
        (
            description = "Unknown error",
            status = INTERNAL_SERVER_ERROR,
        ),
    ),
)]
pub async fn update_threshold_api(
    pool:               PgPool,
    _:                  Identity,
    blueprint_stock_id: BlueprintStockUuid,
    thresholds:         Vec<AddThreshold>,
) -> Result<impl Reply, Rejection> {
    match update_threshold(
        &pool,
        blueprint_stock_id,
        thresholds,
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(BlueprintStockError::ThresholdNotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error updating threshold, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
