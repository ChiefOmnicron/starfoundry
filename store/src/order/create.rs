use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use starfoundry_lib_eve_gateway::ExtractIdentity;
use starfoundry_lib_notification::{Discord, DiscordColor, DiscordEmbed};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::config::{OrderUuid, ProductUuid};
use crate::order::error::{OrderError, Result};

/// Create Order
/// 
/// - Alternative route: `/latest/orders`
/// - Alternative route: `/v1/orders`
/// 
/// ---
/// 
/// Creates a new order
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "orders",
    request_body = CreateOrder,
    responses(
        (
            body = CreateOrderResponse,
            description = "Id of the new order",
            status = CREATED,
        ),
        BadRequest,
        NotFound,
        Unauthorized,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    identity:       ExtractIdentity,
    Json(info):     Json<CreateOrder>,
) -> Result<impl IntoResponse> {
    let order_uuid: Uuid = sqlx::query!("
            INSERT INTO order_info (
                character_id,
                quantity,
                delivery_location,
                comment
            )
            VALUES ($1, $2, $3, $4)
            RETURNING id
        ",
            *identity.character_info.character_id,
            info.quantity,
            info.delivery_location,
            info.comment,
        )
        .fetch_one(&state.postgres)
        .await
        .map(|x| x.id.into())
        .map_err(OrderError::GeneralSqlxError)?;

    sqlx::query!("
            INSERT INTO order_product (
                order_id,
                is_additional,
                name,
                price,
                image_type,
                image_type_id,
                content
            ) VALUES
            (
                $1, FALSE,
                (SELECT name FROM product WHERE id = $2),
                (SELECT price FROM product WHERE id = $2),
                (SELECT image_type FROM product WHERE id = $2),
                (SELECT image_type_id FROM product WHERE id = $2),
                (SELECT content FROM product WHERE id = $2)
            )
        ",
            order_uuid,
            *info.product_uuid,
        )
        .execute(&state.postgres)
        .await
        .map_err(OrderError::GeneralSqlxError)?;

    if let Some(x) = info.additional_option {
        sqlx::query!("
                INSERT INTO order_product (
                    order_id,
                    is_additional,
                    name,
                    price,
                    image_type,
                    image_type_id,
                    content
                ) VALUES
                (
                    $1, TRUE,
                    (SELECT name FROM product WHERE id = $2),
                    (SELECT price FROM product WHERE id = $2),
                    (SELECT image_type FROM product WHERE id = $2),
                    (SELECT image_type_id FROM product WHERE id = $2),
                    (SELECT content FROM product WHERE id = $2)
                )
            ",
                order_uuid,
                *x,
            )
            .execute(&state.postgres)
            .await
            .map_err(OrderError::GeneralSqlxError)?;
    }

    let product = sqlx::query!("
            SELECT name
            FROM product
            WHERE id = $1
        ",
            *info.product_uuid,
        )
        .fetch_one(&state.postgres)
        .await
        .map_err(OrderError::GeneralSqlxError)?;

    let new_order = DiscordEmbed::new(
            "New order",
            "",
            DiscordColor::DarkGreen,
        )
        .add_field("Character", &identity.character_info.character_name)
        .add_field("Product", &product.name)
        .clone();

    match Discord::new()
        .add_embed(new_order)
        .send(state.discord_url.as_ref())
        .await {

        Ok(_)  => tracing::info!("Discord message send"),
        Err(e) => tracing::error!("Failed to send discord message, {e}"),
    }

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateOrderResponse {
                id: order_uuid.into(),
            })
        )
    )
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CreateOrderResponse {
    id: OrderUuid,
}

#[derive(Debug, Deserialize, ToSchema)]
#[cfg_attr(test, derive(serde::Serialize))]
#[schema(
    example = json!({
        "product_uuid": "9ce5d85f-3f2a-4cd9-a970-55bf21bfc151",
        "quantity": 1,
        "delivery_location": "UALX-3",
        "additional_option": null
    })
)]
pub struct CreateOrder {
    product_uuid:       ProductUuid,
    quantity:           i32,
    delivery_location:  String,
    additional_option:  Option<ProductUuid>,
    comment:            Option<String>,
}
