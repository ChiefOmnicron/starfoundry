use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_industry::TagUuid;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::AppState;
use crate::tag::error::Result;
use crate::tag::service::delete;

/// Delete Tag
/// 
/// - Alternative route: `/latest/tags/{TagUuid}`
/// - Alternative route: `/v1/tags/{TagUuid}`
/// 
/// ---
/// 
/// Deletes a tag
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    delete,
    path = "/{TagUuid}",
    tag = "Project",
    params(
        TagUuid,
    ),
    responses(
        (
            description = "Project was deleted",
            status = NO_CONTENT,
        ),
        Unauthorized,
        Forbidden,
        NotFound,
        InternalServerError,
    ),
    security(
        ("api_key" = [])
    ),
)]
pub async fn api(
    State(state):   State<AppState>,
    Path(tag_id):   Path<TagUuid>,
) -> Result<impl IntoResponse> {
    delete(
            &state.postgres,
            tag_id,
        )
        .await?;

    Ok(
        (
            StatusCode::NO_CONTENT,
            Json(())
        )
        .into_response()
    )
}
