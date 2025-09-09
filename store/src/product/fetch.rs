mod service;

pub use self::service::*;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::config::ProductUuid;
use crate::product::error::{ProductError, Result};
use crate::product::Product;
use crate::product::util::{check_blacklist, check_whitelist};
use starfoundry_lib_eve_gateway::ExtractIdentity;

/// Fetch Product
/// 
/// - Alternative route: `/latest/products/{ProductUuid}`
/// - Alternative route: `/v1/products/{ProductUuid}`
/// 
/// ---
/// 
/// Fetches a specific product
/// 
#[utoipa::path(
    get,
    path = "/{ProductUuid}",
    tag = "products",
    params(
        ProductUuid,
    ),
    responses(
        (
            body = Product,
            description = "Requested product",
            status = OK,
        ),
        NotFound,
        Unauthorized,
        Forbidden,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):       State<AppState>,
    Path(product_uuid): Path<ProductUuid>,
    identity:           ExtractIdentity,
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

    if let Some(x) = self::fetch(
        &state.postgres,
        product_uuid,
        character_id,
        corporation_id,
        alliance_id,
    ).await? {
        Ok(
            (
                StatusCode::OK,
                Json(x),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::NOT_FOUND,
                Json(()),
            )
            .into_response()
        )
    }
}
