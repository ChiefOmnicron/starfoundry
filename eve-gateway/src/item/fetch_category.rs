use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::Category;
use starfoundry_lib_types::CategoryId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::services::fetch_category;
use crate::state::AppState;

use crate::item::error::Result;

/// Fetch a category
/// 
/// - Alternative route: `/latest/items/category/{CategoryId}`
/// - Alternative route: `/v1/items/category/{CategoryId}`
/// 
/// ---
/// 
/// Resolves all information about a category
/// 
#[utoipa::path(
    get,
    path = "/category/{CategoryId}",
    tag = "Items",
    params(
        CategoryId,
    ),
    responses(
        (
            body = Category,
            description = "Information about a category",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):      State<AppState>,
    Path(category_id): Path<CategoryId>,
) -> Result<impl IntoResponse> {
    let entry = fetch_category(
        &state.postgres,
        category_id,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
