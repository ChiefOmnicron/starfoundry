use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{fetch_character, EveGatewayClient};
use starfoundry_lib_types::CharacterId;

use crate::config::OrderUuid;
use crate::order::{OrderProduct, OrderResponse};
use crate::product::{ProductError, Result};

pub async fn fetch(
    pool:           &PgPool,
    gateway_client: &impl EveGatewayClient,
    character_id:   CharacterId,
    order_uuid:     OrderUuid,
) -> Result<Option<OrderResponse>> {
    let order = sqlx::query!("
            SELECT
                id,
                character_id,
                quantity,
                status,
                delivery_location,
                comment,
                created_at
            FROM order_info
            WHERE id = $1
            AND character_id = $2
            AND status = ANY('{ACCEPTED, IN_PROGRESS, DONE}')
            ORDER BY created_at
        ",
            *order_uuid,
        *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    let order_info = if let Some(x) = order {
        x
    } else {
        return Ok(None);
    };

    let character_info = fetch_character(
        gateway_client,
        order_info.character_id.into(),
    )
    .await?;

    let products = sqlx::query!("
            SELECT
                name,
                price,
                image_type,
                image_type_id,
                content,
                is_additional
            FROM order_product
            WHERE order_id = $1
        ",
            *order_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(ProductError::GeneralSqlxError)?
        .into_iter()
        .map(|x| {
            OrderProduct {
                name:           x.name,
                price:          x.price,
                image_type:     x.image_type,
                image_type_id:  x.image_type_id.into(),
                content:        x.content,
                is_additional:  x.is_additional,
            }
        })
        .collect::<Vec<_>>();

    let order_info = OrderResponse {
        id:                 order_info.id.into(),
        character:          character_info,
        quantity:           order_info.quantity,
        status:             order_info.status,
        delivery_location:  order_info.delivery_location,
        comment:            order_info.comment,
        ordered_at:         order_info.created_at,

        products,
    };
    Ok(Some(order_info))
}
