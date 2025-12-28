use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, EveGatewayClient};
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::{AppState, SERVICE_NAME};
use crate::order::{OrderProduct, OrderResponse};
use crate::product::error::{ProductError, Result};
use crate::product::util::{check_blacklist, check_whitelist};
use crate::order::create::CreateOrder;

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
    let character_id = identity.character_id;
    let corporation_id = identity.corporation_id;
    let alliance_id = identity.alliance_id;

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

    let character_info = EveGatewayClient::new(SERVICE_NAME.into())?
        .fetch_character(
            character_id.into(),
        )
        .await?;

    let mut orders = sqlx::query!("
            SELECT
                id,
                character_id,
                quantity,
                status,
                delivery_location,
                comment,
                expected_delivery_date,
                created_at
            FROM order_info
            WHERE character_id = $1
            AND status = ANY('{ACCEPTED, IN_PROGRESS, DELIVERED}')
            ORDER BY created_at
        ",
            *identity.character_id,
        )
        .fetch_all(&state.postgres)
        .await
        .map_err(ProductError::GeneralSqlxError)?
        .into_iter()
        .map(|x| {
            OrderResponse {
                id:                     x.id.into(),
                character:              character_info.clone(),
                quantity:               x.quantity,
                status:                 x.status,
                delivery_location:      x.delivery_location,
                comment:                x.comment,
                ordered_at:             x.created_at,
                expected_delivery_date: x.expected_delivery_date,

                products: Vec::new(),
            }
        })
        .collect::<Vec<_>>();

    let mut all_orders = Vec::new();
    for order in orders.iter_mut() {
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
