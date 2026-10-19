use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::GroupId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::Group;
use crate::item::error::{ItemError, Result};
use crate::state::AppState;

/// Fetch Group
///
/// - Alternative route: `/latest/items/group/{GroupId}`
/// - Alternative route: `/v1/items/group/{GroupId}`
///
/// ---
///
/// Resolves all information about a group
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
    State(state): State<AppState>,
    Path(group_id): Path<GroupId>,
) -> Result<impl IntoResponse> {
    let entry = fetch_group(&state.postgres, group_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub async fn fetch_group(
    pool: &PgPool,
    group_id: GroupId,
) -> Result<Option<Group>> {
    let category = sqlx::query!(
        r#"
            SELECT
                group_id,
                category_id,
                name
            FROM groups
            WHERE group_id = $1
        "#,
        *group_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ItemError::FetchGroup(e, group_id))?;

    if let Some(x) = category {
        Ok(Some(Group {
            group_id:    x
                .group_id
                .into(),
            category_id: x
                .category_id
                .into(),
            name:        x.name,
        }))
    } else {
        Ok(None)
    }
}
