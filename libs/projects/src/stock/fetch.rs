use sqlx::PgPool;

use crate::{sort_by_market_group, Error, ProjectUuid, Result, Stock, StockEntry, StockGroup};

pub async fn fetch(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Stock> {
    sqlx::query!(
        r#"
            SELECT
                ps.type_id,
                ps.quantity,
                ps.cost,
                i.name AS "item_name",
                i.category_id,
                i.group_id
            FROM project_stock ps
            JOIN item i
              ON i.type_id = ps.type_id
            WHERE ps.project_id = $1
            ORDER BY i.name
        "#,
            *project_uuid
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchStock(e, project_uuid))
        .map(|rows| {
            rows
                .into_iter()
                .map(|x| StockEntry {
                    item_name:   x.item_name,
                    quantity:    x.quantity,
                    cost:        x.cost,
                    type_id:     x.type_id.into(),
                    category_id: x.category_id.into(),
                    group_id:    x.group_id.into(),
                })
                .collect::<Vec<_>>()
        })
        .map(|x| Stock::new(x))
}

sort_by_market_group!(sort_stock_by_market_group, StockEntry, StockGroup);
