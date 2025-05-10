use sqlx::PgPool;

use crate::{sort_by_market_group, Error, Excess, ExcessEntry, ExcessGroup, ProjectUuid, Result};

pub async fn fetch(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Excess> {
    sqlx::query!(
        r#"
            SELECT
                i.type_id,
                i.name AS "item_name",
                i.category_id,
                i.group_id,
                quantity,
                cost
            FROM project_excess pe
            JOIN item i ON i.type_id = pe.type_id
            WHERE pe.project_id = $1
            ORDER BY item_name
        "#,
            *project_uuid
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchExcess(e, project_uuid))
        .map(|rows| {
            rows
                .into_iter()
                .map(|x| ExcessEntry {
                    quantity:    x.quantity,
                    item_name:   x.item_name,
                    type_id:     x.type_id.into(),
                    category_id: x.category_id.into(),
                    group_id:    x.group_id.into(),
                    cost:        x.cost,
                })
                .collect::<Vec<_>>()
        })
        .map(|x| Excess::new(x))
}

sort_by_market_group!(sort_excess_by_market_group, ExcessEntry, ExcessGroup);
