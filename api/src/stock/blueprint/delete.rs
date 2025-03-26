use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use super::{BlueprintStockError, BlueprintStockUuid};

pub async fn delete(
    pool:               &PgPool,
    character_id:       CharacterId,
    blueprint_stock_id: BlueprintStockUuid,
) -> Result<(), BlueprintStockError> {
    let result = sqlx::query!("
            DELETE FROM stock_blueprints
            WHERE id = $1
            AND owner = $2
        ",
            *blueprint_stock_id,
            *character_id,
        )
        .execute(pool)
        .await
        .map_err(|e| BlueprintStockError::DeleteStock(e, blueprint_stock_id))?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(BlueprintStockError::NotFound(blueprint_stock_id))
    }
}

#[utoipa::path(
    delete,
    operation_id = "stocks_blueprints_delete",
    path = "/api/v1/stocks/blueprints/{blueprintStockId}",
    tag = "blueprint-stocks",
    params(
        (
            "blueprintStockId" = BlueprintStockUuid,
            description = "UUID of the bpc stock to update",
        ),
    ),
    responses(
        (
            description = "The stock was deleted",
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
pub async fn delete_api(
    pool:               PgPool,
    identity:           Identity,
    blueprint_stock_id: BlueprintStockUuid,
) -> Result<impl Reply, Rejection> {
    match delete(
        &pool,
        identity.character_id(),
        blueprint_stock_id,
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(BlueprintStockError::NotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error deleting bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
