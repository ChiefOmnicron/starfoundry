use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::tag::UpdateTag;
use starfoundry_lib_industry::TagUuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized, UnprocessableEntity, UnsupportedMediaType};
use crate::AppState;
use crate::tag::error::Result;
use crate::tag::service::update;

/// Update Tag
/// 
/// - Alternative route: `/v1/tags/{TagUuid}`
/// - Alternative route: `/latest/tags/{TagUuid}`
/// 
/// ---
/// 
/// Updates the tag
/// 
/// ## Security
/// - authenticated
/// - project_group:write
/// 
#[utoipa::path(
    put,
    path = "/{TagUuid}",
    tag = "Tags",
    request_body = UpdateTag,
    params(
        TagUuid,
    ),
    responses(
        (
            description = "The tag was updated",
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
    identity:           ExtractIdentity,
    State(state):       State<AppState>,
    Path(tag_id):       Path<TagUuid>,
    Json(update_info):  Json<UpdateTag>,
) -> Result<impl IntoResponse> {
    update(
        &state.postgres,
        identity.character_id,
        tag_id,
        update_info,
    ).await?;

    Ok((
        StatusCode::NO_CONTENT,
    ))
}
