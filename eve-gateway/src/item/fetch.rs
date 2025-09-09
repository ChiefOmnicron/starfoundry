mod model;
mod service;

pub use self::model::Item;
pub use self::service::fetch;

use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::state::AppState;

use crate::item::error::Result;

/// Fetch an item
/// 
/// - Alternative route: `/latest/items/{TypeId}`
/// - Alternative route: `/v1/items/{TypeId}`
/// 
/// ---
/// 
/// Resolves all information about an item
/// 
#[utoipa::path(
    get,
    path = "/{TypeId}",
    tag = "Items",
    params(
        TypeId,
    ),
    responses(
        (
            body = Item,
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
    let entry = fetch(
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
