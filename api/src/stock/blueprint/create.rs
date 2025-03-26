use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use utoipa::ToSchema;
use uuid::Uuid;
use warp::reject::Rejection;
use warp::reply::Reply;

use crate::{Identity, ReplyError};
use super::{BlueprintStockError, BlueprintStockUuid};

pub async fn create(
    pool:            &PgPool,
    character_id:    CharacterId,
    blueprint_stock: CreateBlueprintStock,
) -> Result<BlueprintStockUuid, BlueprintStockError> {
    sqlx::query!("
            INSERT INTO stock_blueprints (
                name,
                description,
                owner,
                notifications
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id
        ",
            blueprint_stock.name,
            blueprint_stock.description,
            *character_id,
            &blueprint_stock.notifications,
        )
        .fetch_one(pool)
        .await
        .map(|x| BlueprintStockUuid(x.id))
        .map_err(BlueprintStockError::CreateNewStock)
}

#[utoipa::path(
    post,
    operation_id = "stocks_blueprints_create",
    path = "/api/v1/stocks/blueprints",
    tag = "blueprint-stocks",
    request_body = CreateBlueprintStock,
    responses(
        (
            body = BlueprintStockUuid,
            content_type = "application/json",
            description = "UUID of the new entry",
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
            description = "Only application/json is supported",
            status = UNSUPPORTED_MEDIA_TYPE,
        ),
        (
            description = "Unknown error",
            status = INTERNAL_SERVER_ERROR,
        ),
    ),
)]
pub async fn create_api(
    pool:            PgPool,
    identity:        Identity,
    blueprint_stock: CreateBlueprintStock,
) -> Result<impl Reply, Rejection> {
    match create(
        &pool,
        identity.character_id(),
        blueprint_stock,
    ).await {
        Ok(x) => Ok(warp::reply::json(&x)),
        Err(e) => {
            tracing::error!("Unexpected error creating bpc stock, {e}");
            Err(ReplyError::Internal.into())
        },
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateBlueprintStock {
    pub name:          String,
    pub description:   String,
    pub notifications: Vec<Uuid>,
}
