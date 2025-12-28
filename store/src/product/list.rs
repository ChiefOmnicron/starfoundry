use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::{IntoParams, ToSchema};

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::product::error::{ProductError, Result};
use crate::product::util::{check_blacklist, check_whitelist};
use crate::product::Product;
use crate::product::create::AdditionalOption;

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
    params(
        ListProductFilter,
    ),
    responses(
        (
            body = Vec<Product>,
            description = "All products provided",
            status = OK,
        ),
        (
            description = "No products match",
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
    filter:         Query<ListProductFilter>,
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

    let filter_tags: Vec<String> = if let Some(x) = filter.tags.clone() {
        x
            .split(",")
            .map(|x| x.into())
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let products = sqlx::query!("
            SELECT
                id,
                category,
                name,
                price,
                image_type,
                image_type_id,
                description,
                tags,
                content,
                message,
                delivery_time,
                additional_products,
                blacklist,
                whitelist,
                delivery_location
            FROM product
            WHERE
                NOT (LOWER(name) LIKE '%' || LOWER($1) || '%') IS FALSE AND
                NOT (LOWER(category) LIKE '%' || LOWER($2) || '%') IS FALSE AND
                (
                    NOT (tags @> $3::VARCHAR[]) IS FALSE
                )
            ORDER BY name
        ",
            filter.name,
            filter.category,
            &filter_tags,
        )
        .fetch_all(&state.postgres)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    let products = products
        .into_iter()
        .filter(|x| {
            !(
                x.blacklist.contains(&*character_id) ||
                x.blacklist.contains(&*corporation_id.unwrap_or(0.into())) ||
                x.blacklist.contains(&*alliance_id.unwrap_or(0.into()))
            )
        })
        .filter(|x| {
            if identity.is_admin {
                true
            } else if !x.whitelist.is_empty() {
                x.whitelist.contains(&*character_id) ||
                x.whitelist.contains(&*corporation_id.unwrap_or(0.into())) ||
                x.whitelist.contains(&*alliance_id.unwrap_or(0.into()))
            } else {
                true
            }
        })
        .map(|x| {
            let tags = x.tags.unwrap_or_default();

            Product {
                id: x.id.into(),
                category: x.category,
                description: x.description,
                image_type: x.image_type,
                image_type_id: x.image_type_id.into(),
                name: x.name,
                price: x.price as u64,
                tags: tags,
                content: x.content,
                message: x.message,
                delivery_time: x.delivery_time,
                delivery_location: x.delivery_location,
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

#[derive(Debug, Deserialize, IntoParams, ToSchema)]
#[into_params(parameter_in = Query)]
pub struct ListProductFilter {
    pub name:       Option<String>,
    pub category:   Option<String>,
    pub tags:       Option<String>,
}
