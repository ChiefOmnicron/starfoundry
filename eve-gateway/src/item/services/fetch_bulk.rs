use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{Category, Group, Item};
use starfoundry_lib_types::TypeId;

use crate::item::error::{ItemError, Result};

/// Fetches the character information for the given ids from the database.
/// If the character does not exist yet, it will be fetched using the EVE-API.
/// 
pub async fn fetch_item_bulk(
    pool:     &PgPool,
    type_ids: Vec<TypeId>,
) -> Result<Vec<Item>> {
    if type_ids.is_empty() {
        return Ok(Vec::new());
    }

    let mut type_ids = type_ids;
    type_ids.sort();
    type_ids.dedup();

    let type_ids = sqlx::query!(r#"
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
            WHERE type_id = ANY($1)
            ORDER BY name
        "#,
            &type_ids.clone().into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::FetchItemBulk)?
        .into_iter()
        .map(|x| Item {
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
            repackaged: x.repackaged,
        })
        .collect::<Vec<_>>();

    Ok(
        type_ids
    )
}
