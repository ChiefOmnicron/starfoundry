use sqlx::PgPool;
use starfoundry_lib_eve_gateway::Category;
use starfoundry_lib_types::CategoryId;

use crate::item::error::{ItemError, Result};

pub async fn fetch_category(
    pool:        &PgPool,
    category_id: CategoryId,
) -> Result<Option<Category>> {
    let category = sqlx::query!(r#"
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
            category_id: x.category_id.into(),
            name:        x.name,
        }))
    } else {
        Ok(None)
    }
}
