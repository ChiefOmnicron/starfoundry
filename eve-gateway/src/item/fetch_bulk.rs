use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::Item;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::services::fetch_item_bulk;
use crate::state::AppState;
use crate::item::error::Result;

/// Bulk Fetch an item
/// 
/// - Alternative route: `/latest/items`
/// - Alternative route: `/v1/items`
/// 
/// ---
/// 
/// Bulk resolve items by their type_id
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "Items",
    request_body = Vec<TypeId>,
    responses(
        (
            body = Vec<Item>,
            description = "Information about an item",
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
    let entry = fetch_item_bulk(
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
