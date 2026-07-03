use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::tag::{CreateTag, CreateTagResponse};

use crate::api_docs::{BadRequest, InternalServerError, Unauthorized};
use crate::AppState;
use crate::tag::error::Result;
use crate::tag::service::create;

/// Create Tag
/// 
/// - Alternative route: `/latest/tags`
/// - Alternative route: `/v1/tags`
/// 
/// ---
/// 
/// Creates a new tag
/// 
/// ## Security
/// - authenticated
/// - project_group:read
/// 
#[utoipa::path(
    post,
    path = "/",
    tag = "tags",
    request_body = CreateTag,
    responses(
        (
            body = CreateTagResponse,
            description = "UUID of the new tag",
            status = OK,
        ),
        (
            description = "There aren't any projects matching the filter",
            status = NO_CONTENT,
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
    identity:       ExtractIdentity,
    State(state):   State<AppState>,
    Json(tag_info): Json<CreateTag>,
) -> Result<impl IntoResponse> {
    let id = create(
            &state.postgres,
            identity.character_id,
            tag_info,
        ).await?;

    Ok(
        (
            StatusCode::CREATED,
            Json(CreateTagResponse {
                id: id.into(),
            })
        )
    )
}
