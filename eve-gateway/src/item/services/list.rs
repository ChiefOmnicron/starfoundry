use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{Category, Group, Item, ListItemFilter};

use crate::item::error::{ItemError, Result};

pub async fn list_items(
    pool:   &PgPool,
    filter: ListItemFilter,
) -> Result<Vec<Item>> {
    if let Some(true) = filter.blueprint {
        blueprint(pool, filter.name).await
    } else if let Some(true) = filter.buildable {
        buildable(pool, filter.name).await
    } else {
        all(pool, filter.name).await
    }
}

async fn all(
    pool: &PgPool,
    name: String,
) -> Result<Vec<Item>> {
    let items = sqlx::query!(r#"
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
            WHERE (LOWER(i.name) LIKE '%' || LOWER($1) || '%')
                -- exclude unnecessary categories
                AND i.category_id != ALL(ARRAY[2, 11, 14, 30, 63, 91, 2118, 350001])
            ORDER BY i.name ASC
            LIMIT 10
        "#,
            name,
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::ListItem)?
        .into_iter()
        .map(|x| {
            Item {
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
            }
        })
        .collect::<Vec<_>>();
    Ok(items)
}

async fn blueprint(
    pool: &PgPool,
    name: String,
) -> Result<Vec<Item>> {
    let items = sqlx::query!(r#"
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
            FROM blueprint_json bsjon
            JOIN item i ON i.type_id = bsjon.btype_id
            JOIN category c ON i.category_id = c.category_id
            JOIN groups g ON i.group_id = g.group_id
            WHERE (LOWER(i.name) LIKE '%' || LOWER($1) || '%blueprint')
            ORDER BY i.name ASC
            LIMIT 10
        "#,
            name,
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::ListItem)?
        .into_iter()
        .map(|x| {
            Item {
                category:      Category {
                    category_id: x.category_id.into(),
                    name:        x.category_name
                },
                group:         Group {
                    group_id:    x.group_id.into(),
                    category_id: x.category_id.into(),
                    name:        x.group_name,
                },
                name:          x.name,
                type_id:       x.type_id.into(),
                volume:        x.volume,

                meta_group: x.meta_group_id.map(Into::into),
                repackaged:    x.repackaged,
            }
        })
        .collect::<Vec<_>>();
    Ok(items)
}

async fn buildable(
    pool: &PgPool,
    name: String,
) -> Result<Vec<Item>> {
    let items = sqlx::query!(r#"
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
            FROM blueprint_json bsjon
            JOIN item i ON i.type_id = bsjon.ptype_id
            JOIN category c ON i.category_id = c.category_id
            JOIN groups g ON i.group_id = g.group_id
            WHERE (LOWER(i.name) LIKE '%' || LOWER($1) || '%')
            ORDER BY i.name ASC
            LIMIT 10
        "#,
            name,
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::ListItem)?
        .into_iter()
        .map(|x| {
            Item {
                category:      Category {
                    category_id: x.category_id.into(),
                    name:        x.category_name
                },
                group:         Group {
                    group_id:    x.group_id.into(),
                    category_id: x.category_id.into(),
                    name:        x.group_name,
                },
                name:          x.name,
                type_id:       x.type_id.into(),
                volume:        x.volume,

                meta_group: x.meta_group_id.map(Into::into),
                repackaged:    x.repackaged,
            }
        })
        .collect::<Vec<_>>();
    Ok(items)
}
