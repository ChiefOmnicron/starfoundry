use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::api_docs::{ErrorResponse, Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::config::OrderUuid;
use crate::product::error::{ProductError, Result};

/// List orders
/// 
/// - Alternative route: `/latest/admin/orders/{OrderUuid}/status`
/// - Alternative route: `/v1/admin/orders/{OrderUuid}/status`
/// 
/// ---
/// 
/// Updates the status of an order
/// 
/// ## Security
/// - authenticated
/// - admin
/// 
#[utoipa::path(
    put,
    path = "/orders/{OrderUuid}/status",
    tag = "admin",
    request_body = Status,
    responses(
        (
            description = "The update was successful",
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
    identity:         ExtractIdentity,
    State(state):     State<AppState>,
    Path(order_uuid): Path<OrderUuid>,
    Json(status):     Json<Status>,
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

    sqlx::query!("
            UPDATE order_info
            SET status = $2
            WHERE id = $1
        ",
            *order_uuid,
            status.status,
        )
        .execute(&state.postgres)
        .await
        .map_err(ProductError::GeneralSqlxError)?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(()),
        )
        .into_response()
    )
}

// TODO: document possible options
#[derive(Debug, Deserialize, ToSchema)]
pub struct Status {
    status: String,
}
