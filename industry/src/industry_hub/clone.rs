use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use serde::Serialize;
use starfoundry_lib_gateway::ExtractIdentity;
use utoipa::ToSchema;

use crate::api_docs::{Forbidden, InternalServerError, Unauthorized};
use crate::AppState;
use crate::industry_hub::error::Result;
use crate::industry_hub::IndustryHubUuid;
use crate::industry_hub::service::clone;

/// Clone Industry Hub
/// 
/// - Alternative route: `/latest/industry-hubs/{IndustryHubUuid}/clone`
/// - Alternative route: `/v1/industry-hubs/{IndustryHubUuid}/clone`
/// 
/// ---
/// 
/// Clones a industry hub
/// 
/// ## Security
/// - authenticated
/// - industry_hub:shared
/// 
#[utoipa::path(
    put,
    path = "/{IndustryHubUuid}/clone",
    tag = "Industry Hubs",
    params(
        IndustryHubUuid,
    ),
    responses(
        (
            body = IndustryHubCloneResponse,
            description = "The structure group was successfully cloned",
            status = OK
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
    identity:                ExtractIdentity,
    State(state):            State<AppState>,
    Path(industry_hub_uuid): Path<IndustryHubUuid>,
) -> Result<impl IntoResponse> {
    let data = clone(
            &state.pool,
            identity.character_id,
            industry_hub_uuid,
        )
        .await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(IndustryHubCloneResponse {
                id: data,
            }),
        )
        .into_response()
    )
}

#[derive(Debug, Serialize, ToSchema)]
pub struct IndustryHubCloneResponse {
    id: IndustryHubUuid,
}
