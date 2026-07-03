use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use starfoundry_lib_gateway::ExtractIdentity;
use starfoundry_lib_industry::tag::Tag;
use starfoundry_lib_industry::TagUuid;

use crate::AppState;
use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::tag::error::Result;
use crate::tag::service::fetch;

/// Fetch Tag
/// 
/// - Alternative route: `/latest/tags/{TagUuid}`
/// - Alternative route: `/v1/tags/{TagUuid}`
/// 
/// ---
/// 
/// Fetches information about a tag
/// 
/// ## Security
/// - authenticated
/// - structure:read
/// 
#[utoipa::path(
    get,
    path = "/{TagUuid}",
    tag = "Tags",
    params(
        TagUuid,
    ),
    responses(
        (
            body = Tag,
            description = "Information about the structure",
            status = OK,
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
    identity:       ExtractIdentity,
    State(state):   State<AppState>,
    Path(tag_id):   Path<TagUuid>,
) -> Result<impl IntoResponse> {
    let entry = fetch(
            &state.postgres,
            identity.character_id,
            tag_id,
        )
        .await?;

    if let Some(x) = entry {
        Ok(
            (
                StatusCode::OK,
                Json(x)
            )
            .into_response()
        )
    } else {
        Ok(
            (
                StatusCode::NO_CONTENT,
                Json(())
            )
            .into_response()
        )
    }
}
