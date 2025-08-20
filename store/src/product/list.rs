use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::product::error::{ProductError, Result};
use crate::product::util::{check_blacklist, check_whitelist};
use crate::product::Product;
use crate::product::create::AdditionalOption;
use starfoundry_lib_eve_gateway::ExtractIdentity;

/// List Products
/// 
/// - Alternative route: `/latest/products`
/// - Alternative route: `/v1/products`
/// 
/// ---
/// 
/// Lists all available products
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "products",
    responses(
        (
            body = Vec<Product>,
            description = "All s provided",
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

    let products = sqlx::query!("
            SELECT
                uuid,
                category,
                name,
                price,
                image_type,
                image_type_id,
                description,
                tags,
                content,
                additional_products
            FROM product
            ORDER BY name
        ")
        .fetch_all(&state.postgres)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    let products = products
        .into_iter()
        .map(|x| {
            let mut tags = x.tags.unwrap_or_default();
            tags.sort();

            Product {
                category: x.category,
                description: x.description,
                image_type: x.image_type,
                image_type_id: x.image_type_id.into(),
                name: x.name,
                price: x.price as u64,
                tags: tags,
                uuid: x.uuid.into(),
                content: x.content,
                additional_options: x.additional_products
                    .unwrap_or_default()
                    .into_iter()
                    .map(|x| AdditionalOption {
                        reference_id: x.into(),
                    })
                    .collect::<Vec<_>>(),
            }
        })
        .collect::<Vec<_>>();

    if products.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(products),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(products),
            )
            .into_response()
        )
    }
}
