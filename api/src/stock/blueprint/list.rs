use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use uuid::Uuid;
use warp::{Reply, Rejection};

use crate::{ReplyError, Identity};
use super::error::BlueprintStockError;

pub async fn list(
    pool:         &PgPool,
    character_id: CharacterId,
) -> Result<Vec<Uuid>, BlueprintStockError> {
    sqlx::query!(r#"
            SELECT id
            FROM stock_blueprints
            WHERE
                owner = $1
                ORDER BY name
        "#,
            *character_id,
        )
        .fetch_all(pool)
        .await
        .map_err(BlueprintStockError::ListStocks)
        .map(|ids| {
            ids
                .into_iter()
                .map(|x| x.id)
                .collect::<Vec<_>>()
        })
}

#[utoipa::path(
    get,
    operation_id = "stocks_blueprint_list",
    path = "/api/v1/stocks/blueprints",
    tag = "blueprint-stocks",
    responses(
        (
            body = Vec<Uuid>,
            content_type = "application/json",
            description = "List of all Blueprint Stocks that are configured",
            status = OK,
        ),
        (
            description = "Invalid parameter",
            status = BAD_REQUEST,
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
pub async fn list_api(
    pool:     PgPool,
    identity: Identity,
) -> Result<impl Reply, Rejection> {
    match list(
        &pool,
        identity.character_id(),
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(e) => {
            tracing::error!("Unexpected error listing structures, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}
