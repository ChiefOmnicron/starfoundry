mod service;

pub use self::service::*;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{ExtractIdentity, MtlsApiClient};

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::config::OrderUuid;
use crate::product::error::Result;
use crate::product::Product;

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
    identity:         ExtractIdentity,
    State(state):     State<AppState>,
    Path(order_uuid): Path<OrderUuid>,
) -> Result<impl IntoResponse> {
    if let Some(x) = self::fetch(
        &state.postgres,
        &MtlsApiClient::new()?,
        identity.character_id,
        order_uuid,
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
