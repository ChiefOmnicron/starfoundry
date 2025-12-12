use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::{Item, ListItemFilter};

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::services::list_items;
use crate::state::AppState;

use crate::item::error::Result;

/// Fetch an item
/// 
/// - Alternative route: `/latest/items`
/// - Alternative route: `/v1/items`
/// 
/// ---
/// 
/// Resolves all information about all items
/// 
#[utoipa::path(
    get,
    path = "/",
    tag = "Items",
    params(
        TypeId,
    ),
    params(ListItemFilter),
    responses(
        (
            body = Vec<Item>,
            description = "Information about all items",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):  State<AppState>,
    Query(filter): Query<ListItemFilter>,
) -> Result<impl IntoResponse> {
    let entry = list_items(
        &state.postgres,
        filter,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
