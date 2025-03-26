use sqlx::PgPool;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use super::{BlueprintStockThreshold, BlueprintStockUuid};
use super::error::BlueprintStockError;

pub async fn fetch_threshold(
    pool:               &PgPool,
    blueprint_stock_id: BlueprintStockUuid,
) -> Result<Vec<BlueprintStockThreshold>, BlueprintStockError> {
    let result = sqlx::query!("
        SELECT id
        FROM stock_blueprints
        WHERE id = $1
    ",
        *blueprint_stock_id
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| BlueprintStockError::FetchById(e, blueprint_stock_id))?;

    if result.is_none() {
        return Err(BlueprintStockError::NotFound(blueprint_stock_id));
    }

    sqlx::query!(r#"
            SELECT *
            FROM stock_blueprint_thresholds
            WHERE blueprint_stock_id = $1
        "#,
            *blueprint_stock_id
        )
        .fetch_all(pool)
        .await
        .map(|x| {
            x.into_iter()
                .map(|y| BlueprintStockThreshold {
                    id:       Some(BlueprintStockUuid(y.id)),
                    type_id:  y.type_id.into(),
                    want:     y.want,
                    critical: y.critical,
                    min_runs: y.min_runs,
                    min_me:   y.min_me,
                    min_te:   y.min_te,
                })
                .collect::<Vec<_>>()
        })
        .map_err(|e| BlueprintStockError::FetchThresholds(
            e,
            blueprint_stock_id
        ))
}

#[utoipa::path(
    get,
    operation_id = "stocks_blueprints_thresholds_fetch",
    path = "/api/v1/stocks/blueprints/{blueprintStockId}/thresholds",
    tag = "blueprint-stocks",
    params(
        (
            "blueprintStockId" = BlueprintStockUuid,
            description = "UUID of the bpc stock to fetch",
        ),
    ),
    responses(
        (
            body = Vec<BlueprintStockThreshold>,
            content_type = "application/json",
            description = "All thresholds configured for the stock",
            status = OK,
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
pub async fn fetch_thresholds_api(
    pool:               PgPool,
    _:                  Identity,
    blueprint_stock_id: BlueprintStockUuid,
) -> Result<impl Reply, Rejection> {
    match fetch_threshold(
        &pool,
        blueprint_stock_id,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(BlueprintStockError::NotFound(bpc_stock_id)) => {
            tracing::warn!("blueprint stock id not found {bpc_stock_id}");
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error fetching bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
