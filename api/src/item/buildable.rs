use serde::Serialize;
use sqlx::PgPool;
use starfoundry_libs_types::{TypeId, CategoryId, GroupId};
use tracing::instrument;

use super::ItemError;

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
pub async fn buildable(
    pool: &PgPool,
) -> Result<Vec<ItemResponse>, ItemError> {
    sqlx::query_as!(
        ItemResponse,
        r#"
            SELECT
                bsjon.ptype_id AS "type_id!",
                i.category_id  AS "category_id!",
                i.group_id     AS "group_id!",
                i.volume       AS "volume!",
                i.name         AS "name!",
                i.base_price
            FROM blueprint_json bsjon
            JOIN item i ON i.type_id = bsjon.ptype_id
            ORDER BY i.name
        "#)
        .fetch_all(pool)
        .await
        .map_err(ItemError::FetchBuildable)
}

#[derive(Debug, Serialize)]
pub struct ItemResponse {
    pub type_id:     TypeId,
    pub category_id: CategoryId,
    pub group_id:    GroupId,
    pub volume:      f32,
    pub name:        String,

    pub base_price:  Option<f32>,
}
