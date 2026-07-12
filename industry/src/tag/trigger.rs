use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::tag::UpdateTag;
use starfoundry_lib_industry::TagUuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::tag::error::Result;
use crate::tag::service::trigger;

/// Trigger
/// 
/// - Alternative route: `/v1/tags`
/// - Alternative route: `/latest/tags`
/// 
/// ---
/// 
/// Triggers the auto tag generation
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/",
    tag = "Tags",
    request_body = UpdateTag,
    params(
        TagUuid,
    ),
    responses(
        (
            description = "Tags were updated",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        UnsupportedMediaType,
        UnprocessableEntity,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    identity:       ExtractIdentity,
    State(state):   State<AppState>,
) -> Result<impl IntoResponse> {
    trigger(
        &state.postgres,
        identity.character_id,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
