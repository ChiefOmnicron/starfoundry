use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use utoipa::ToSchema;
use warp::http::StatusCode;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use super::{BlueprintStockError, BlueprintStockUuid};
use uuid::Uuid;

pub async fn update(
    pool:               &PgPool,
    character_id:       CharacterId,
    blueprint_stock_id: BlueprintStockUuid,
    blueprint_stock:    UpdateBlueprintStock,
) -> Result<(), BlueprintStockError> {
    let result = sqlx::query!("
            UPDATE stock_blueprints
            SET name = $3,
                description = $4,
                notifications = $5
            WHERE id = $1
            AND owner = $2
        ",
            *blueprint_stock_id,
            *character_id,
            blueprint_stock.name,
            blueprint_stock.description,
            &blueprint_stock.notifications,
        )
        .execute(pool)
        .await
        .map_err(|e| BlueprintStockError::UpdateStock(e, blueprint_stock_id))?;

    if result.rows_affected() > 0 {
        Ok(())
    } else {
        Err(BlueprintStockError::NotFound(blueprint_stock_id))
    }
}

/// /stocks/blueprints/{blueprintStockId}
/// 
#[utoipa::path(
    put,
    operation_id = "stocks_blueprints_update",
    path = "/stocks/blueprints/{blueprintStockId}",
    tag = "blueprint-stocks",
    params(
        (
            "blueprintStockId" = BlueprintStockUuid,
            description = "UUID of the bpc stock to update",
        ),
    ),
    request_body = UpdateBlueprintStock,
    responses(
        (
            description = "Entry updated",
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
pub async fn update_api(
    pool:               PgPool,
    identity:           Identity,
    blueprint_stock_id: BlueprintStockUuid,
    blueprint_stock:    UpdateBlueprintStock,
) -> Result<impl Reply, Rejection> {
    match update(
        &pool,
        identity.character_id(),
        blueprint_stock_id,
        blueprint_stock,
    ).await {
        Ok(_) => Ok(warp::reply::with_status(
            warp::reply::json(&()),
            StatusCode::NO_CONTENT,
        )),
        Err(BlueprintStockError::NotFound(_)) => {
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error updating bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateBlueprintStock {
    pub name:          String,
    pub description:   String,
    pub notifications: Vec<Uuid>,
}
