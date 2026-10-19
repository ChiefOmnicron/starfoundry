use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::{Category, Group, Item};
use crate::item::error::{ItemError, Result};
use crate::state::AppState;

/// Fetch Item
///
/// - Alternative route: `/latest/items/{TypeId}`
/// - Alternative route: `/v1/items/{TypeId}`
///
/// ---
///
/// Resolves all information about an item
#[utoipa::path(
    get,
    path = "/{TypeId}",
    tag = "Items",
    params(
        TypeId,
    ),
    responses(
        (
            body = Item,
            description = "Information about an item",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(type_id): Path<TypeId>,
) -> Result<impl IntoResponse> {
    let entry = fetch_item(&state.postgres, type_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub async fn fetch_item(
    pool: &PgPool,
    type_id: TypeId,
) -> Result<Option<Item>> {
    let item = sqlx::query!(
        r#"
            SELECT
                type_id,
                volume,
                meta_group_id,
                repackaged,
                i.category_id,
                i.group_id,
                i.name,
                c.name AS "category_name",
                g.name AS "group_name"
            FROM item i
            JOIN category c ON i.category_id = c.category_id
            JOIN groups g ON i.group_id = g.group_id
            WHERE type_id = $1
        "#,
        *type_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ItemError::Fetch(e, type_id))?;

    if let Some(x) = item {
        Ok(Some(Item {
            category: Category {
                category_id: x
                    .category_id
                    .into(),
                name:        x.category_name,
            },
            group:    Group {
                group_id:    x
                    .group_id
                    .into(),
                category_id: x
                    .category_id
                    .into(),
                name:        x.group_name,
            },
            name:     x.name,
            type_id:  x
                .type_id
                .into(),
            volume:   x.volume,

            meta_group: x
                .meta_group_id
                .map(Into::into),
            repackaged: x.repackaged,
        }))
    } else {
        Ok(None)
    }
}
