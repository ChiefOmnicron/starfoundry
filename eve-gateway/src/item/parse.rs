use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::ParseResult;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::error::Result;
use crate::item::services::{load_items, parse};
use crate::state::AppState;

/// Fetch an item
/// 
/// - Alternative route: `/latest/items/parse`
/// - Alternative route: `/v1/items/parse`
/// 
/// ---
/// 
/// Attempts to parse a blob of items
/// 
#[utoipa::path(
    post,
    path = "/parse",
    tag = "Items",
    request_body = String,
    responses(
        (
            body = Vec<ParseResult>,
            description = "Result of the parser",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):  State<AppState>,
    Json(content): Json<String>,
) -> Result<impl IntoResponse> {
    let item_cache = load_items(
            &state.postgres,
        )
        .await?;

    let entry = parse(
        &item_cache,
        &content,
    );

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
