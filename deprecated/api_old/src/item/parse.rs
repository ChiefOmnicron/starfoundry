use sqlx::PgPool;
use starfoundry_lib_items::{ParsedItem, parse, load_items};
use tracing::instrument;

use super::ItemError;

#[instrument(err, skip(pool), level = "error")]
pub async fn parse_items(
    pool:       &PgPool,
    content:    String,
) -> Result<Vec<ParsedItem>, ItemError> {
    let item_cache = load_items(pool)
        .await
        .map_err(ItemError::BuildupItemCache)?;

    let result = parse(
        &item_cache,
        &content,
    );
    Ok(result.items)
}
