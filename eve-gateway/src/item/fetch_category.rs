use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use starfoundry_lib_types::CategoryId;

use crate::api_docs::{InternalServerError, NotFound};
use crate::asset::Category;
use crate::item::error::{ItemError, Result};
use crate::state::AppState;

/// Fetch Category
///
/// - Alternative route: `/latest/items/category/{CategoryId}`
/// - Alternative route: `/v1/items/category/{CategoryId}`
///
/// ---
///
/// Resolves all information about a category
#[utoipa::path(
    get,
    path = "/category/{CategoryId}",
    tag = "Items",
    params(
        CategoryId,
    ),
    responses(
        (
            body = Category,
            description = "Information about a category",
            status = OK,
        ),
        NotFound,
        InternalServerError,
    ),
)]
pub async fn api(
    State(state): State<AppState>,
    Path(category_id): Path<CategoryId>,
) -> Result<impl IntoResponse> {
    let entry = fetch_category(&state.postgres, category_id).await?;

    Ok((StatusCode::OK, Json(entry)).into_response())
}

pub async fn fetch_category(
    pool: &PgPool,
    category_id: CategoryId,
) -> Result<Option<Category>> {
    let category = sqlx::query!(
        r#"
            SELECT
                category_id,
                name
            FROM category
            WHERE category_id = $1
        "#,
        *category_id,
    )
    .fetch_optional(pool)
    .await
    .map_err(|e| ItemError::FetchCategory(e, category_id))?;

    if let Some(x) = category {
        Ok(Some(Category {
            category_id: x
                .category_id
                .into(),
            name:        x.name,
        }))
    } else {
        Ok(None)
    }
}
