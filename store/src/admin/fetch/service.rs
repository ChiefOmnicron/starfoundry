use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;

use crate::admin::fetch::model::AdminOrderResponse;
use crate::config::OrderUuid;
use crate::order::OrderProduct;
use crate::product::{ProductError, Result};

pub async fn fetch(
    pool:           &PgPool,
    api_client:     &impl EveGatewayApiClient,
    order_uuid:     OrderUuid,
) -> Result<Option<AdminOrderResponse>> {
    let order = sqlx::query!("
            SELECT
                id,
                character_id,
                quantity,
                status,
                delivery_location,
                comment,
                expected_delivery_date,
                sf_industry_link,
                created_at
            FROM order_info
            WHERE id = $1
            AND status != 'CANCELED'
            ORDER BY created_at
        ",
            *order_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    let order_info = if let Some(x) = order {
        x
    } else {
        return Ok(None);
    };

    let character_info = api_client
        .fetch_character(
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

    let order_info = AdminOrderResponse {
        id:                     order_info.id.into(),
        character:              character_info,
        quantity:               order_info.quantity,
        status:                 order_info.status,
        delivery_location:      order_info.delivery_location,
        comment:                order_info.comment,
        ordered_at:             order_info.created_at,
        sf_industry_link:       order_info.sf_industry_link,
        expected_delivery_date: order_info.expected_delivery_date,

        products,
    };
    Ok(Some(order_info))
}
