use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{Category, Group, Item};
use starfoundry_lib_types::TypeId;

use crate::item::error::{ItemError, Result};

pub async fn fetch_item(
    pool:    &PgPool,
    type_id: TypeId,
) -> Result<Option<Item>> {
    let item = sqlx::query!(r#"
            SELECT
                type_id,
                volume,
                meta_group_id,
                repackaged,
                i.category_id,
                i.group_id,
                i.name,
                c.name AS "category_name",
                g.name AS "group_name"
            FROM item i
            JOIN category c ON i.category_id = c.category_id
            JOIN groups g ON i.group_id = g.group_id
            WHERE type_id = $1
        "#,
            *type_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| ItemError::FetchItem(e, type_id))?;

    if let Some(x) = item {
        Ok(Some(Item {
            category:      Category {
                category_id: x.category_id.into(),
                name:        x.category_name
            },
            group:         Group {
                group_id:    x.group_id.into(),
                category_id: x.category_id.into(),
                name:        x.group_name,
            },
            name:    x.name,
            type_id: x.type_id.into(),
            volume:  x.volume,

            meta_group: x.meta_group_id.map(Into::into),
            repackaged:    x.repackaged,
        }))
    } else {
        Ok(None)
    }
}
