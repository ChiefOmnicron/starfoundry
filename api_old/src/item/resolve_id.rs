use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_libs_types::TypeId;
use tracing::instrument;

use super::{Item, ItemError};

// TODO: is this not a search?
#[instrument(err, skip(pool), level = "error")]
pub async fn resolve_id(
    pool:    &PgPool,
    type_id: TypeId,
) -> Result<Option<Item>, ItemError> {
    let entry = sqlx::query!(r#"
            SELECT
                i.type_id     AS "type_id!",
                i.category_id AS "category_id!",
                i.group_id    AS "group_id!",
                i.volume      AS "volume!",
                i.name        AS "name!",
                i.base_price
            FROM item i
            WHERE i.type_id = $1
        "#,
            *type_id
        )
        .fetch_optional(pool)
        .await
        .map_err(ItemError::ResolveId)?
        .map(|x| Item {
            type_id:     x.type_id.into(),
            category_id: x.category_id.into(),
            group_id:    x.group_id.into(),
            volume:      x.volume.into(),
            name:        x.name,
            base_price:  x.base_price,
        });
    Ok(entry)
}

#[derive(Debug, Deserialize)]
pub struct ResolveIdNameFilter {
    pub is_buildable: Option<bool>
}
