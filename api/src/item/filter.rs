use sqlx::PgPool;
use tracing::instrument;
use serde::Deserialize;

use super::{ItemError, Item};

/// Gets a list of all item names that can be constructed.
/// 
/// # Errors
/// 
/// If the database access failes.
/// 
/// # Returns
/// 
/// List of all items that have a blueprint associated with them.
/// 
#[instrument(err, skip(pool), level = "error")]
pub async fn all(
    pool: &PgPool,
) -> Result<Vec<Item>, ItemError> {
    let blueprints = sqlx::query_as!(
        Item,
        r#"
            SELECT
                type_id,
                category_id,
                group_id,
                volume,
                name,
                base_price
            FROM items
            -- Exclude some of the categories that we won´t need
            WHERE category_id != ALL(ARRAY[30, 63, 91, 2118])
            ORDER BY name
        "#)
        .fetch_all(pool)
        .await
        .map_err(ItemError::FetchAll)?;

    Ok(blueprints)
}

#[derive(Debug, Deserialize)]
pub struct ItemFilterRequest {
    buildable: bool
}
