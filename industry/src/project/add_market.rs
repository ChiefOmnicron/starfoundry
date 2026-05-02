use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::ProjectUuid;

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::project::error::Result;
use crate::project::service::{AddMarketEntryRequest, add_market};

/// List Groups
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/market`
/// - Alternative route: `/v1/projects/{ProjectUuid}/market`
/// 
/// ---
/// 
/// Adds additional market entries.
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    post,
    path = "/{ProjectUuid}/market",
    tag = "projects",
    request_body = AddMarketEntryRequest,
    responses(
        (
            description = "The entries were added",
            status = CREATED,
        ),
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):       State<AppState>,
    Path(project_id):   Path<ProjectUuid>,
    Json(entries):      Json<Vec<AddMarketEntryRequest>>,
) -> Result<impl IntoResponse> {
    add_market(
            &state.postgres,
            project_id,
            entries,
        ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(())
        )
    )
}
