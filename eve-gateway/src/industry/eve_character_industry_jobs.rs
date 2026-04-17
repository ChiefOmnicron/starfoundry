use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::IndustryJob;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CorporationId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_corporation_auth;

const SCOPE: &str = "esi-industry.read_corporation_jobs.v1";

/// Fetch Player Market
/// 
/// - Alternative route: `/latest/eve/industry/corporation`
/// - Alternative route: `/v1/eve/industry/corporation`
/// 
/// ---
/// 
/// Fetches the running corporation industry jobs
/// 
#[utoipa::path(
    get,
    path = "/industry/jobs/corporation",
    tag = "Industry",
    params(
        CorporationId,
    ),
    responses(
        (
            body = Vec<IndustryJob>,
            description = "Corporation market data",
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
    let api_client = api_client_corporation_auth(
            &state.postgres,
            state.eve_api_metric,
            identity.host()?,
            identity.character_id,
            identity.corporation_id.unwrap_or(CorporationId(0)),
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
        "latest/corporations/{}/industry_jobs",
        // TODO: refactor
        *identity.corporation_id.unwrap_or(CorporationId(0)),
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
