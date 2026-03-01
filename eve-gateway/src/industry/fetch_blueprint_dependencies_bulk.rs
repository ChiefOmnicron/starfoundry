use axum::extract::State;
use axum::Json;
use axum::response::IntoResponse;
use reqwest::StatusCode;
use starfoundry_lib_eve_gateway::BlueprintDependency;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::industry::error::Result;
use crate::state::AppState;
use crate::industry::service::fetch_blueprint_dependency_bulk;

/// Fetch System Index
/// 
/// - Alternative route: `/latest/industry/blueprints/dependencies/bulk`
/// - Alternative route: `/v1/industry/blueprints/dependencies/bulk`
/// 
/// ---
/// 
/// Loads all open orders from a character
/// 
#[utoipa::path(
    post,
    path = "/blueprints/dependencies/bulk",
    tag = "Industry",
    responses(
        (
            body = Vec<BlueprintDependency>,
            description = "Blueprint and their dependencies",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Json(type_ids): Json<Vec<TypeId>>,
) -> Result<impl IntoResponse> {
    let blueprint_dependencies = fetch_blueprint_dependency_bulk(
            &state.postgres,
            type_ids,
        )
        .await?;

    if blueprint_dependencies.is_empty() {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(blueprint_dependencies),
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::OK,
                Json(blueprint_dependencies),
            )
            .into_response()
        )
    }
}
