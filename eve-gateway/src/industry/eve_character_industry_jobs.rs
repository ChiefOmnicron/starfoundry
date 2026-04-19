use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::IndustryJob;
use starfoundry_lib_gateway::ExtractIdentity;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_auth;

const SCOPE: &str = "esi-industry.read_character_jobs.v1";

/// Fetch Player Market
/// 
/// - Alternative route: `/latest/eve/industry/character`
/// - Alternative route: `/v1/eve/industry/character`
/// 
/// ---
/// 
/// Fetches the running character industry jobs
/// 
#[utoipa::path(
    get,
    path = "/industry/jobs/character",
    tag = "Industry",
    responses(
        (
            body = Vec<IndustryJob>,
            description = "Character industry jobs",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    identity:       ExtractIdentity,
    State(state):   State<AppState>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_auth(
            &state.postgres,
            state.eve_api_metric,
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
        "latest/character/{}/industry/jobs",
        identity.character_id,
    );
    let job_data = api_client
        .fetch_page_auth::<IndustryJob>(&path)
        .await?;

    if job_data.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(job_data),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(job_data),
            )
            .into_response()
        )
    }
}
