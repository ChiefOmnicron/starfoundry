use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;

use crate::api_docs::{Forbidden, InternalServerError, NotFound, Unauthorized};
use crate::tag::error::{Result, TagError};
use crate::{AppState, TagUuid};

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
    State(state): State<AppState>,
    Path(tag_id): Path<TagUuid>,
) -> Result<impl IntoResponse> {
    delete(&state.postgres, tag_id).await?;

    Ok((StatusCode::NO_CONTENT, Json(())).into_response())
}

pub async fn delete(
    pool: &PgPool,
    tag_id: TagUuid,
) -> Result<()> {
    sqlx::query!(
        r#"
            DELETE FROM tag
            WHERE id = $1
        "#,
        *tag_id,
    )
    .execute(pool)
    .await
    .map(drop)
    .map_err(|e| TagError::Delete(e, tag_id))
}
