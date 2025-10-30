use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_eve_gateway::Group;
use starfoundry_lib_types::GroupId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::services::fetch_group;
use crate::state::AppState;

use crate::item::error::Result;

/// Fetch a group
/// 
/// - Alternative route: `/latest/items/group/{GroupId}`
/// - Alternative route: `/v1/items/group/{GroupId}`
/// 
/// ---
/// 
/// Resolves all information about a group
/// 
#[utoipa::path(
    get,
    path = "/group/{GroupId}",
    tag = "Items",
    params(
        GroupId,
    ),
    responses(
        (
            body = Group,
            description = "Information about a group",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Path(group_id): Path<GroupId>,
) -> Result<impl IntoResponse> {
    let entry = fetch_group(
        &state.postgres,
        group_id,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}
