use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{Category, Group, Item, ListItemFilter};

use crate::item::error::{ItemError, Result};

pub async fn list_items(
    pool:   &PgPool,
    filter: ListItemFilter,
) -> Result<Vec<Item>> {
    if let Some(true) = filter.blueprint {
        blueprint(pool, filter).await
    } else if let Some(true) = filter.buildable {
        buildable(pool, filter).await
    } else {
        all(pool, filter).await
    }
}

async fn all(
    pool:   &PgPool,
    filter: ListItemFilter,
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
            WHERE
                NOT (LOWER(i.name) LIKE '%' || LOWER($1) || '%') IS FALSE AND
                NOT ($2::INTEGER[] IS NULL OR i.group_id = ANY($2)) IS FALSE AND
                NOT ($3::INTEGER[] IS NULL OR i.category_id = ANY($3)) IS FALSE AND
                -- exclude unnecessary categories
                i.category_id != ALL(ARRAY[2, 11, 14, 30, 63, 91, 2118, 350001])
            ORDER BY i.name ASC
            LIMIT $4
        "#,
            filter.name,
            &filter.groups.map(|x| x.into_iter().map(|y| *y).collect::<Vec<_>>()) as _,
            &filter.categories.map(|x| x.into_iter().map(|y| *y).collect::<Vec<_>>()) as _,
            filter.limit,
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::List)?
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
    pool:  &PgPool,
    filter: ListItemFilter,
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
            JOIN item i ON i.type_id = bsjon.blueprint_type_id
            JOIN category c ON i.category_id = c.category_id
            JOIN groups g ON i.group_id = g.group_id
            WHERE
                (LOWER(i.name) LIKE '%' || LOWER($1) || '%blueprint') IS FALSE AND
                NOT ($2::INTEGER[] IS NULL OR i.group_id = ANY($2)) IS FALSE AND
                NOT ($3::INTEGER[] IS NULL OR i.category_id = ANY($3)) IS FALSE
            ORDER BY i.name ASC
            LIMIT $4
        "#,
            filter.name,
            &filter.groups.map(|x| x.into_iter().map(|y| *y).collect::<Vec<_>>()) as _,
            &filter.categories.map(|x| x.into_iter().map(|y| *y).collect::<Vec<_>>()) as _,
            filter.limit,
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::List)?
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
    pool:   &PgPool,
    filter: ListItemFilter,
) -> Result<Vec<Item>> {
    let items = sqlx::query!(r#"
            SELECT
                i.type_id,
                i.volume,
                i.meta_group_id,
                i.repackaged,
                i.category_id,
                i.group_id,
                i.name,
                c.name AS "category_name",
                g.name AS "group_name"
            FROM blueprint_json bsjon
            JOIN item i ON i.type_id = bsjon.product_type_id
            JOIN category c ON i.category_id = c.category_id
            JOIN groups g ON i.group_id = g.group_id
            JOIN blueprint b ON b.type_id = bsjon.blueprint_type_id
            WHERE
                NOT (LOWER(i.name) LIKE '%' || LOWER($1) || '%') IS FALSE AND
                NOT ($2::INTEGER[] IS NULL OR i.group_id = ANY($2)) IS FALSE AND
                NOT ($3::INTEGER[] IS NULL OR i.category_id = ANY($3)) IS FALSE AND
                NOT ($4::INTEGER[] IS NULL OR b.required_service_type_id = ANY($4)) IS FALSE
            ORDER BY i.name ASC
            LIMIT $5
        "#,
            filter.name,
            &filter.groups.map(|x| x.into_iter().map(|y| *y).collect::<Vec<_>>()) as _,
            &filter.categories.map(|x| x.into_iter().map(|y| *y).collect::<Vec<_>>()) as _,
            &filter.services.map(|x| x.into_iter().map(|y| *y).collect::<Vec<_>>()) as _,
            filter.limit,
        )
        .fetch_all(pool)
        .await
        .map_err(ItemError::List)?
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
