mod service;

pub use self::service::*;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::EveGatewayClient;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{ErrorResponse, Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::config::OrderUuid;
use crate::product::error::Result;
use crate::order::OrderResponse;

/// Fetch Product
/// 
/// - Alternative route: `/latest/admin/orders/{OrderUuid}`
/// - Alternative route: `/v1/admin/orders/{OrderUuid}`
/// 
/// ---
/// 
/// Fetches a specific product
/// 
/// ## Security
/// - authenticated
/// - admin
/// 
#[utoipa::path(
    get,
    path = "/orders/{OrderUuid}",
    tag = "admin",
    params(
        OrderUuid,
    ),
    responses(
        (
            body = OrderResponse,
            description = "Requested order",
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

    if let Some(x) = self::fetch(
        &state.postgres,
        &EveGatewayClient::new()?,
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
