use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Deserialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::config::OrderUuid;
use crate::order::error::{OrderError, Result};
use crate::AppState;

/// Updates an order
/// 
/// - Alternative route: `/latest/orders/{OrderUuid}`
/// - Alternative route: `/v1/orders/{OrderUuid}`
/// 
/// ---
/// 
/// Lists all available orders
/// 
/// ## Security
/// - authenticated
/// 
#[utoipa::path(
    put,
    path = "/{OrderUuid}",
    tag = "orders",
    request_body = UpdateOrder,
    responses(
        (
            description = "The order was updated",
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
    State(state):     State<AppState>,
    Path(order_uuid): Path<OrderUuid>,
    identity:         ExtractIdentity,
    Json(data):       Json<UpdateOrder>,
) -> Result<impl IntoResponse> {
    let result = sqlx::query!("
            UPDATE order_info
            SET comment = $3
            WHERE id = $1
            AND character_id = $2
        ",
            *order_uuid,
            *identity.character_id,
            data.comment
        )
        .execute(&state.postgres)
        .await
        .map_err(OrderError::GeneralSqlxError)?;

    if result.rows_affected() > 0 {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(()),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::UNAUTHORIZED,
            )
            .into_response()
        )
    }
}

#[derive(Debug, Deserialize, ToSchema)]
#[cfg_attr(test, derive(serde::Serialize))]
#[schema(
    example = json!({
        "comment": "My cool new comment"
    })
)]
pub struct UpdateOrder {
    comment: String,
}
