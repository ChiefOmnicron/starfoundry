use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::item::{Item, ItemError};
use crate::state::AppState;

use crate::item::error::Result;

/// Fetch an item
/// 
/// - Alternative route: `/latest/items/{TypeId}`
/// - Alternative route: `/v1/items/{TypeId}`
/// 
/// ---
/// 
/// Resolves all information about an item
/// 
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
    State(state):  State<AppState>,
    Path(type_id): Path<TypeId>,
) -> Result<impl IntoResponse> {
    let entry = fetch_item(
        &state.postgres,
        type_id,
    ).await?;

    Ok(
        (
            StatusCode::OK,
            Json(entry)
        )
        .into_response()
    )
}

pub async fn fetch_item(
    pool:    &PgPool,
    type_id: TypeId,
) -> Result<Option<Item>> {
    let item = sqlx::query!("
            SELECT
                type_id,
                category_id,
                group_id,
                volume,
                name,
                meta_group_id,
                repackaged
            FROM item
            WHERE type_id = $1
            ORDER BY name
        ",
            *type_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ItemError::FetchItem(e, type_id))?;

    if let Some(x) = item {
        Ok(Some(Item {
            category_id:   x.category_id.into(),
            group_id:      x.group_id.into(),
            name:          x.name,
            type_id:       x.type_id.into(),
            volume:        x.volume,

            meta_group_id: x.meta_group_id.map(Into::into),
            repackaged:    x.repackaged,
        }))
    } else {
        Ok(None)
    }
}
