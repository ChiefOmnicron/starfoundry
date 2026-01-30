use axum::extract::Path;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::contract::PublicContractItem;
use starfoundry_lib_types::ContractId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::eve_client::EveApiClient;

/// Fetch Market for a region
/// 
/// - Alternative route: `/latest/contracts/public/{ContractId}/items`
/// - Alternative route: `/v1/contracts/public/{ContractId}/items`
/// 
/// ---
/// 
/// Returns a list of all items in the given contract
/// 
#[utoipa::path(
    get,
    path = "/public/{ContractId}/items",
    tag = "Contracts",
    params(
        ContractId,
    ),
    responses(
        (
            body = Vec<PublicContractItem>,
            description = "List of all items in the contract",
            status = OK,
        ),
        (
            description = "No items in the contract",
            status = NO_CONTENT,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    Path(contract_id): Path<ContractId>,
) -> Result<impl IntoResponse> {
    let api_client = EveApiClient::new()?;

    let path = format!("latest/contracts/public/items/{}", *contract_id);
    let contract_data = api_client
        .fetch_page::<PublicContractItem>(&path)
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

