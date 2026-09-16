use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::Reprocessing;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::error::Result;
use crate::item::services::fetch_bulk_reprocessing;
use crate::state::AppState;

/// Bulk Fetch Reprocessing
/// 
/// - Alternative route: `/latest/items/reprocessing`
/// - Alternative route: `/v1/items/reprocessing`
/// 
/// ---
/// 
/// Bulk information about reprocessing
/// 
#[utoipa::path(
    post,
    path = "/reprocessing",
    tag = "Items",
    request_body = Vec<TypeId>,
    responses(
        (
            body = HashMap<TypeId, Vec<Reprocessing>>,
            description = "Reprocessing Information about an item",
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
    let entry = fetch_bulk_reprocessing(
        &state.postgres,
        type_ids,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
