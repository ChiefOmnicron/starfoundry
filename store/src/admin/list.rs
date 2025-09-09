use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{fetch_bulk_character, ExtractIdentity};
use std::collections::HashMap;

use crate::api_docs::{ErrorResponse, Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::order::{Order, OrderProduct, OrderResponse};
use crate::product::error::{ProductError, Result};

/// List orders
/// 
/// - Alternative route: `/latest/admin/orders`
/// - Alternative route: `/v1/admin/orders`
/// 
/// ---
/// 
/// Lists all available orders
/// 
/// ## Security
/// - authenticated
/// - admin
/// 
#[utoipa::path(
    get,
    path = "/orders",
    tag = "admin",
    responses(
        (
            body = Vec<OrderResponse>,
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
    if !identity.is_admin {
        return Ok((
            StatusCode::UNAUTHORIZED,
            Json(
                ErrorResponse {
                    error: "UNAUTHORIZED".into(),
                    description: "Login and try again".into(),
                }
            )
        ).into_response())
    }

    let orders = sqlx::query!("
            SELECT
                id,
                character_id,
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
        .map_err(ProductError::GeneralSqlxError)?
        .into_iter()
        .map(|x| {
            Order {
                id:                 x.id.into(),
                character_id:       x.character_id.into(),
                quantity:           x.quantity,
                status:             x.status,
                delivery_location:  x.delivery_location,
                comment:            x.comment,
                ordered_at:         x.created_at,

                products: Vec::new(),
            }
        })
        .collect::<Vec<_>>();

    let mut character_ids = orders
        .iter()
        .map(|x| x.character_id)
        .collect::<Vec<_>>();
    character_ids.sort();
    character_ids.dedup();
    let character_ids = fetch_bulk_character(
            &identity.gateway_client()?,
            character_ids,
        )
        .await?
        .into_iter()
        .map(|x| (x.character_id, x))
        .collect::<HashMap<_, _>>();

    let mut order_response = Vec::new();
    for order in orders {
        let character_info = if let Some(x) = character_ids.get(&order.character_id) {
            x
        } else {
            continue;
        };

        order_response.push(OrderResponse {
            character:          character_info.clone(),
            id:                 order.id.into(),
            quantity:           order.quantity,
            status:             order.status,
            delivery_location:  order.delivery_location,
            comment:            order.comment,
            ordered_at:         order.ordered_at,

            products:           Vec::new(),
        });
    }

    let mut all_orders = Vec::new();
    for order in order_response.iter_mut() {
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
                ORDER BY is_additional
            ",
                *order.id,
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

        order.products = products;
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

