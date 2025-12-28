use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::Market;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::StructureId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-markets.structure_markets.v1";

/// Fetch Market for a region
/// 
/// - Alternative route: `/latest/market/player/{StructureId}`
/// - Alternative route: `/v1/market/player/{StructureId}`
/// 
/// ---
/// 
/// Resolves the market data for the given region
/// 
#[utoipa::path(
    get,
    path = "/player/{StructureId}",
    tag = "Market",
    params(
        StructureId,
    ),
    responses(
        (
            body = Vec<Market>,
            description = "Market data for the player structure",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(structure_id): Path<StructureId>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_auth(
            &state.postgres,
            identity.host,
            identity.character_id,
            identity.corporation_id,
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

    let path = format!("latest/markets/structures/{structure_id}");
    let market_data = api_client
        .fetch_page_auth::<Market>(&path)
        .await?;

    if market_data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
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
