use serde::Serialize;
use sqlx::PgPool;
use tracing::instrument;

use super::ItemError;

#[instrument(err, skip(pool), level = "error")]
pub async fn resolve_bulk_names(
    pool:  &PgPool,
    names: Vec<String>,
) -> Result<Vec<BulkResolveNamesResponse>, ItemError> {
    sqlx::query_as!(
        BulkResolveNamesResponse,
        r#"
            SELECT
                i.type_id     AS "type_id!",
                i.category_id AS "category_id!",
                i.group_id    AS "group_id!",
                i.volume      AS "volume!",
                i.name        AS "name!",
                i.base_price
            FROM item i
            WHERE name = ANY($1::VARCHAR[])
        "#,
            &names
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::BulkResolveName)
}

#[derive(Debug, Serialize)]
pub struct BulkResolveNamesResponse {
    pub type_id:     i32,
    pub category_id: i32,
    pub group_id:    i32,
    pub volume:      f32,
    pub name:        String,
    pub base_price:  Option<f32>,
}
