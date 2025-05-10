use crate::{Error, Finance, ProjectUuid, Result};
use sqlx::PgPool;

pub async fn fetch(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
) -> Result<Option<Finance>> {
    let sell_price = sqlx::query!("
            SELECT sell_price
            FROM project
            WHERE id = $1
        ",
            *project_uuid
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchFinance(e, project_uuid))?
        .map(|x| x.sell_price.unwrap_or_default());

    let excess = sqlx::query!("
            SELECT SUM(cost) AS excess
            FROM project_excess
            WHERE project_id = $1
        ",
            *project_uuid
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchFinance(e, project_uuid))?
        .map(|x| x.excess.unwrap_or_default());

    let jobs = sqlx::query!("
            SELECT SUM(cost) AS jobs
            FROM project_job
            WHERE project_id = $1
        ",
            *project_uuid
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchFinance(e, project_uuid))?
        .map(|x| x.jobs.unwrap_or_default());

    let market = sqlx::query!("
            SELECT SUM(cost) AS market
            FROM project_market
            WHERE project_id = $1
        ",
            *project_uuid
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchFinance(e, project_uuid))?
        .map(|x| x.market.unwrap_or_default());

    let misc = sqlx::query!("
            SELECT SUM(cost) AS misc
            FROM project_misc
            WHERE project_id = $1
        ",
            *project_uuid
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchFinance(e, project_uuid))?
        .map(|x| x.misc.unwrap_or_default());

    let stocks = sqlx::query!("
            SELECT SUM(cost) AS stocks
            FROM project_stock
            WHERE project_id = $1
        ",
            *project_uuid
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchFinance(e, project_uuid))?
        .map(|x| x.stocks.unwrap_or_default());

    Ok(
        Some(
            Finance {
                sell_price: sell_price.unwrap_or_default(),
                excess:     excess.unwrap_or_default(),
                jobs:       jobs.unwrap_or_default(),
                market:     market.unwrap_or_default(),
                misc:       misc.unwrap_or_default(),
                stock:      stocks.unwrap_or_default(),
            }
        )
    )
}
