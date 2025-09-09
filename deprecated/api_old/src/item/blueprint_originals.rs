use sqlx::PgPool;
use tracing::instrument;

use super::{Item, ItemError};

/// Attempts to find all original blueprints
#[instrument(err, skip(pool), level = "error")]
pub async fn blueprint_originals(
    pool: &PgPool,
) -> Result<Vec<Item>, ItemError> {
    sqlx::query_as!(
        Item,
        "
            SELECT
                type_id,
                category_id,
                group_id,
                volume,
                name,
                base_price
            FROM item
            WHERE category_id = 9
            AND (
                meta_group_id = 1
            OR meta_group_id = 54
            OR meta_group_id IS NULL
            )
            AND base_price IS NOT NULL
            AND category_id != ALL(ARRAY[
                8
            ])
            AND group_id != ALL(ARRAY[
                1993
            ])
            AND type_id != ALL(ARRAY[
                40311
            ])
            ORDER BY NAME ASC
        ")
        .fetch_all(pool)
        .await
        .map_err(ItemError::FetchBlueprintOriginals)
}
