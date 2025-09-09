use sqlx::PgPool;
use starfoundry_lib_types::TypeId;

use crate::item::fetch::model::Item;
use crate::item::error::{ItemError, Result};

pub async fn fetch(
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
