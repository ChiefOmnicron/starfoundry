use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_client::EveApiClient;
use starfoundry_lib_eve_gateway::IndustrySystem;

use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::Result;
use crate::state::AppState;

/// Fetch System Index
/// 
/// - Alternative route: `/latest/eve/industry/system-index`
/// - Alternative route: `/v1/eve/industry/system-index`
/// 
/// ---
/// 
/// Loads all open orders from a character
/// 
/// TODO: change path
#[utoipa::path(
    get,
    path = "/industry/system-index",
    tag = "Industry",
    responses(
        (
            body = Vec<IndustrySystem>,
            description = "Index for the system",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
) -> Result<impl IntoResponse> {
    let api_client = EveApiClient::new(state.eve_api_metric)?;

    let path = format!(
        "latest/industry/systems",
    );
    let system_index_data = api_client
        .fetch_page::<IndustrySystem>(&path)
        .await?;

    if system_index_data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(system_index_data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(system_index_data),
            )
            .into_response()
        )
    }
}
