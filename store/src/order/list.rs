use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::order::{Order, OrderProduct};
use crate::product::error::{ProductError, Result};
use crate::product::util::{check_blacklist, check_whitelist};
use crate::order::create::CreateOrder;
use starfoundry_lib_eve_gateway::ExtractIdentity;

/// List orders
/// 
/// - Alternative route: `/latest/orders`
/// - Alternative route: `/v1/orders`
/// 
/// ---
/// 
/// Lists all available orders
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "orders",
    responses(
        (
            body = Vec<CreateOrder>,
            description = "All orders",
            status = OK,
        ),
        (
            description = "No offer available for the requester",
            status = NO_CONTENT
        ),
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    identity:       ExtractIdentity,
) -> Result<impl IntoResponse> {
    let character_id = identity.character_info.character_id;
    let corporation_id = identity.character_info.corporation_id;
    let alliance_id = identity.character_info.alliance_id;

    if !state
        .shop_config
        .restriction
        .blacklist
        .is_empty() {

        if check_blacklist(
            character_id,
            corporation_id,
            alliance_id,
            state.shop_config.restriction.blacklist.clone(),
        ) {
            return Err(ProductError::Forbidden(character_id));
        }
    }

    if !state
        .shop_config
        .restriction
        .whitelist
        .is_empty() {

        if !check_whitelist(
            character_id,
            corporation_id,
            alliance_id,
            state.shop_config.restriction.whitelist.clone(),
        ) {
            return Err(ProductError::Forbidden(character_id));
        }
    }

    let orders = sqlx::query!("
            SELECT
                uuid,
                quantity,
                status,
                delivery_location,
                comment,
                created_at
            FROM order_info
            ORDER BY created_at
        ")
        .fetch_all(&state.postgres)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    let mut all_orders = Vec::new();
    for order in orders {
        let products = sqlx::query!("
                SELECT
                    name,
                    price,
                    image_type,
                    image_type_id,
                    content,
                    is_additional
                FROM order_product
                WHERE order_uuid = $1
                ORDER BY is_additional
            ",
                order.uuid,
            )
            .fetch_all(&state.postgres)
            .await
            .map_err(ProductError::GeneralSqlxError)?
            .into_iter()
            .map(|x| OrderProduct {
                name: x.name,
                price: x.price,
                image_type: x.image_type,
                image_type_id: x.image_type_id.into(),
                content: x.content,
                is_additional: x.is_additional,
            })
            .collect::<Vec<_>>();

        let order = Order {
            uuid: order.uuid.into(),
            quantity: order.quantity,
            status: order.status,
            delivery_location: order.delivery_location,
            comment: order.comment,
            ordered_at: order.created_at,

            products,
        };
        all_orders.push(order);
    }

    if all_orders.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(all_orders),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(all_orders),
            )
            .into_response()
        )
    }
}
