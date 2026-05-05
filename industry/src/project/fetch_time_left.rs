use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::ProjectUuid;

use crate::{AppState, eve_gateway_api_client};
use crate::api_docs::{BadRequest, InternalServerError, NotFound, Unauthorized};
use crate::project::error::Result;
use crate::project::service::fetch_time_left;

/// Fetch Time Left
/// 
/// - Alternative route: `/latest/projects/{ProjectUuid}/time-left`
/// - Alternative route: `/v1/projects/{ProjectUuid}/time-left`
/// 
/// ---
/// 
/// Fetches an estimate how much time is left in the project until it's completion
/// 
/// ## Security
/// - authenticated
/// - project:read
/// 
#[utoipa::path(
    get,
    path = "/{ProjectUuid}/time-left",
    tag = "projects",
    params(
        ProjectUuid,
    ),
    responses(
        (
            body = i64,
            description = "End date in milliseconds",
            status = OK,
        ),
        NotFound,
        BadRequest,
        Unauthorized,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(project_id):   Path<ProjectUuid>,
) -> Result<impl IntoResponse> {
    let data = fetch_time_left(
            &state.postgres,
            identity.character_id,
            project_id,
            &eve_gateway_api_client()?,
        ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(data),
        )
        .into_response()
    )
}
