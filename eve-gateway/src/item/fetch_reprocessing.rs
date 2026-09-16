use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::Reprocessing;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::error::Result;
use crate::item::services::fetch_reprocessing;
use crate::state::AppState;

/// Fetch Reprocessing
/// 
/// - Alternative route: `/latest/items/{TypeId}/reprocessing`
/// - Alternative route: `/v1/items/{TypeId}/reprocessing`
/// 
/// ---
/// 
/// Resolves reprocessing information about an item
/// 
#[utoipa::path(
    get,
    path = "/{TypeId}/reprocessing",
    tag = "Items",
    params(
        TypeId,
    ),
    responses(
        (
            body = Vec<Reprocessing>,
            description = "Information about an item",
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
    let entry = fetch_reprocessing(
        &state.postgres,
        type_id,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
