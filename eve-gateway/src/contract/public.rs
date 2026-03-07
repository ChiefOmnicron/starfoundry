use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_client::EveApiClient;
use starfoundry_lib_eve_gateway::contract::PublicContract;
use starfoundry_lib_types::RegionId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::state::AppState;

/// Fetch Market for a region
/// 
/// - Alternative route: `/latest/contracts/public/region/{RegionId}`
/// - Alternative route: `/v1/contracts/public/region/{RegionId}`
/// 
/// ---
/// 
/// Returns the cost of all items
/// 
#[utoipa::path(
    get,
    path = "/public/region/{RegionId}",
    tag = "Contracts",
    params(
        RegionId,
    ),
    responses(
        (
            body = Vec<PublicContract>,
            description = "List of all public contracts for the given region",
            status = OK,
        ),
        (
            description = "No public contracts in the region",
            status = NO_CONTENT,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):    State<AppState>,
    Path(region_id): Path<RegionId>,
) -> Result<impl IntoResponse> {
    let api_client = EveApiClient::new(state.eve_api_metric)?;

    let path = format!("latest/contracts/public/{}", *region_id);
    let contract_data = api_client
        .fetch_page::<PublicContract>(&path)
        .await?;

    if contract_data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(contract_data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(contract_data),
            )
            .into_response()
        )
    }
}

