use sqlx::PgPool;

use crate::{sort_by_market_group, Error, Market, MarketEntry, MarketGroup, ProjectUuid, Result};

pub async fn fetch(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Market> {
    sqlx::query_as!(
        MarketEntry,
        r#"
            SELECT
                id,
                i.name       AS "item_name",
                quantity,
                i.type_id,
                i.category_id,
                i.group_id,
                cost,
                source
            FROM project_market pm
            JOIN items i
              ON i.type_id = pm.type_id
            WHERE pm.project_id = $1
            ORDER BY item_name
        "#,
            *project_uuid,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchMarket(e, project_uuid))
        .map(|x| Market::new(x))
}

sort_by_market_group!(sort_market_by_market_group, MarketEntry, MarketGroup);
