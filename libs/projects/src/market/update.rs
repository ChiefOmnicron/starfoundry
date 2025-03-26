use sqlx::PgPool;

use crate::{Error, ProjectMarketUuid, ProjectUuid, Result, UpdateMarket};

pub async fn update(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    market_uuid:  ProjectMarketUuid,
    update:       UpdateMarket,
) -> Result<()> {
    sqlx::query!("
            UPDATE project_market
              SET quantity   = $3,
                  cost       = $4,
                  source     = $5
            WHERE project_id = $1
              AND id = $2
        ",
            *project_uuid,
            *market_uuid,
            &update.quantity,
            &update.cost as _,
            &update.source as _,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateMarket(e, project_uuid))
}
