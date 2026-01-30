use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::Blueprint;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_types::CorporationId;

use crate::api_docs::{InternalServerError, Unauthorized};
use crate::market::error::Result;
use crate::state::AppState;
use crate::utils::api_client_corporation_auth;

const SCOPE: &str = "esi-corporations.read_blueprints.v1";

/// Fetch Player Market
/// 
/// - Alternative route: `/latest/eve/corporations/{CorporationId}/assets/blueprints`
/// - Alternative route: `/v1/eve/corporations/{CorporationId}/assets/blueprints`
/// 
/// ---
/// 
/// Resolves the market data for the given region
/// 
#[utoipa::path(
    get,
    path = "/eve/{CorporationId}/assets/blueprints",
    tag = "Assets",
    params(
        CorporationId,
    ),
    responses(
        (
            body = Vec<Blueprint>,
            description = "Corporation blueprints",
            status = OK,
        ),
        (
            body = Vec<Blueprint>,
            description = "The corporation has no blueprints",
            status = NO_CONTENT,
        ),
        Unauthorized,
        InternalServerError,
    ),
)]
pub async fn fetch_corporation_blueprint_api(
    identity:             ExtractIdentity,
    State(state):         State<AppState>,
    Path(corporation_id): Path<CorporationId>,
) -> Result<impl IntoResponse> {
    let api_client = api_client_corporation_auth(
            &state.postgres,
            identity.host()?,
            identity.character_id,
            corporation_id,
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

    let path = format!("latest/corporations/{corporation_id}/blueprints");
    let blueprints = api_client
        .fetch_page_auth::<Blueprint>(&path)
        .await?;

    if blueprints.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(blueprints),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(blueprints),
            )
            .into_response()
        )
    }
}
