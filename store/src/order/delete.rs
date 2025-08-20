use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::config::OrderUuid;
use crate::order::error::{OrderError, Result};

/// Create Order
/// 
/// - Alternative route: `/latest/orders/{OrderUuid}`
/// - Alternative route: `/v1/orders/{OrderUuid}`
/// 
/// ---
/// 
/// Deletes an order
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    delete,
    path = "/{OrderUuid}",
    tag = "orders",
    params(
        OrderUuid,
    ),
    responses(
        (
            description = "The resource was deleted",
            status = NO_CONTENT,
        ),
        BadRequest,
        NotFound,
        Unauthorized,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):     State<AppState>,
    Path(order_uuid): Path<OrderUuid>,
) -> Result<impl IntoResponse> {
    sqlx::query!("
            DELETE FROM order_info
            WHERE uuid = $1
        ",
            *order_uuid,
        )
        .execute(&state.postgres)
        .await
        .map_err(OrderError::GeneralSqlxError)?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(())
        )
    )
}
