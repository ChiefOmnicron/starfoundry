use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use super::{BlueprintStock, BlueprintStockUuid};
use super::error::BlueprintStockError;

pub async fn fetch(
    pool:               &PgPool,
    character_id:       CharacterId,
    blueprint_stock_id: BlueprintStockUuid,
) -> Result<Option<BlueprintStock>, BlueprintStockError> {
    let blueprints = sqlx::query!(r#"
            SELECT
                id,
                name,
                description,
                notifications
            FROM stock_blueprints
            WHERE owner = $1
            AND id = $2
        "#,
            *character_id,
            *blueprint_stock_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| BlueprintStockError::FetchById(
            e,
            blueprint_stock_id
        ))?;

    let blueprint = if let Some(x) = blueprints {
        x
    } else {
        return Ok(None);
    };

    Ok(Some(
        BlueprintStock {
            id:            Some(blueprint_stock_id),
            description:   blueprint.description,
            name:          blueprint.name,
            notifications: blueprint.notifications.unwrap_or_default(),
        }
    ))
}

/// /stocks/blueprints/{blueprintStockId}
/// 
#[utoipa::path(
    get,
    operation_id = "stocks_blueprints_fetch",
    path = "/stocks/blueprints/{blueprintStockId}",
    tag = "blueprint-stocks",
    params(
        (
            "blueprintStockId" = BlueprintStockUuid,
            description = "UUID of the bpc stock to fetch",
        ),
    ),
    responses(
        (
            body = BlueprintStock,
            content_type = "application/json",
            description = "General information about the stock",
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
pub async fn fetch_api(
    pool:               PgPool,
    identity:           Identity,
    blueprint_stock_id: BlueprintStockUuid,
) -> Result<impl Reply, Rejection> {
    match fetch(
        &pool,
        identity.character_id(),
        blueprint_stock_id,
    ).await {
        Ok(Some(x)) => Ok(warp::reply::json(&x)),
        Ok(None)    => Err(ReplyError::Forbidden.into()),
        Err(BlueprintStockError::NotFound(bpc_stock_id)) => {
            tracing::warn!("bpc stock id not found {bpc_stock_id}");
            Err(ReplyError::Forbidden.into())
        },
        Err(e) => {
            tracing::error!("Unexpected error fetching bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
