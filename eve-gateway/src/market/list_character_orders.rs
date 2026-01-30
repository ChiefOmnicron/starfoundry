use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::market::Market;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-markets.read_character_orders.v1";

/// Fetch Character Orders
/// 
/// - Alternative route: `/latest/market/orders/characters`
/// - Alternative route: `/v1/market/orders/characters`
/// 
/// ---
/// 
/// Loads all open orders from a character
/// 
#[utoipa::path(
    get,
    path = "/orders/characters",
    tag = "Market",
    responses(
        (
            body = Vec<Market>,
            description = "List of orders from the character",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    identity:        ExtractIdentity,
    State(state):    State<AppState>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_auth(
            &state.postgres,
            identity.host()?,
            identity.character_id,
            vec![
                SCOPE.into(),
            ],
        )
        .await?;

    let api_client = if let Some(x) = api_client {
        x
    } else {
        return Ok(
            (
                StatusCode::UNAUTHORIZED,
            )
            .into_response()
        )
    };

    let path = format!(
        "latest/characters/{}/orders",
        *identity.character_id,
    );
    let market_data = api_client
        .fetch_page_auth::<Market>(&path)
        .await?;

    if market_data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(market_data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(market_data),
            )
            .into_response()
        )
    }
}
