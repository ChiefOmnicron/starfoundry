use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;
use crate::structure::error::Result;
use crate::structure::services::fetch_rig_blueprint_bonus;

/// Fetch structure services
/// 
/// - Alternative route: `/latest/structures/rigs/blueprints`
/// - Alternative route: `/v1/structures/rigs/blueprints`
/// 
/// ---
/// 
/// Returns all services that can be installed in the structure
/// 
#[utoipa::path(
    post,
    path = "/rigs/blueprints",
    tag = "Structures",
    params(
        TypeId,
    ),
    responses(
        (
            body = Vec<Item>,
            description = "List of blueprints that are improved by the given rig",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):       State<AppState>,
    Json(rig_type_ids): Json<Vec<TypeId>>,
) -> Result<impl IntoResponse> {
    let entries = fetch_rig_blueprint_bonus(
        &state.postgres,
        rig_type_ids,
    ).await?;

    if entries.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(entries)
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(entries)
            )
            .into_response()
        )
    }
}
