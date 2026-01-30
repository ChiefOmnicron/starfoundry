use axum::extract::{Path, State};
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::BlueprintJson;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::Result;
use crate::industry::service::fetch_blueprint_json;
use crate::state::AppState;

/// Fetch System Index
/// 
/// - Alternative route: `/latest/industry/blueprint/{TypeId}/json`
/// - Alternative route: `/v1/industry/blueprint/{TypeId}/json`
/// 
/// ---
/// 
/// Loads all open orders from a character
/// 
#[utoipa::path(
    get,
    path = "/blueprints/{TypeId}/json",
    tag = "Industry",
    responses(
        (
            body = BlueprintJson,
            description = "JSON representing blueprint dependencies",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):  State<AppState>,
    Path(type_id): Path<TypeId>,
) -> Result<impl IntoResponse> {
    let blueprint_json = fetch_blueprint_json(
            &state.postgres,
            type_id,
        )
        .await?;

    if blueprint_json.is_none() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(blueprint_json),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(blueprint_json),
            )
            .into_response()
        )
    }
}
