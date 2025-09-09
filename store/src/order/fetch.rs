mod service;

pub use self::service::*;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::auth::ExtractIdentity;
use crate::config::OrderUuid;
use crate::product::error::{ProductError, Result};
use crate::product::Product;
use crate::product::util::{check_blacklist, check_whitelist};

/// Fetch Product
/// 
/// - Alternative route: `/latest/products/{OrderUuid}`
/// - Alternative route: `/v1/products/{OrderUuid}`
/// 
/// ---
/// 
/// Fetches a specific product
/// 
#[utoipa::path(
    get,
    path = "/{OrderUuid}",
    tag = "products",
    params(
        OrderUuid,
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
    State(state):              State<AppState>,
    ExtractIdentity(identity): ExtractIdentity,
    Path(product_uuid):        Path<OrderUuid>,
) -> Result<impl IntoResponse> {
    let character_id = identity.character_id();
    let corporation_id = identity.corporation_id().await?;
    let alliance_id = identity.alliance_id().await?;

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

    if let Some(x) = self::fetch(&state.pool, product_uuid).await? {
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
