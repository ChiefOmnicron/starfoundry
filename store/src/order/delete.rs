use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, EveGatewayClient};
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_notification::{Discord, DiscordColor, DiscordEmbed};

use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::{AppState, SERVICE_NAME};
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
    identity:         ExtractIdentity,
    State(state):     State<AppState>,
    Path(order_uuid): Path<OrderUuid>,
) -> Result<impl IntoResponse> {
    sqlx::query!("
            UPDATE order_info
            SET status = 'CANCELED'
            WHERE id = $1
            AND character_id = $2
        ",
            *order_uuid,
            *identity.character_id,
        )
        .execute(&state.postgres)
        .await
        .map_err(OrderError::GeneralSqlxError)?;

    let character_info = EveGatewayClient::new(SERVICE_NAME.into())?
        .fetch_character(
            identity.character_id,
        )
        .await?;

    let order_canceled = DiscordEmbed::new(
            "Order canceled",
            "",
            DiscordColor::DarkRed,
        )
        .add_field("Character", &character_info.character_name)
        .clone();

    match Discord::new()
        .add_embed(order_canceled)
        .send(state.discord_url.as_ref())
        .await {

        Ok(_)  => tracing::info!("Discord message send"),
        Err(e) => tracing::error!("Failed to send discord message, {e}"),
    }

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(())
        )
    )
}
