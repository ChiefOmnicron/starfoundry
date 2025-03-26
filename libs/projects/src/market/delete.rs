use sqlx::PgPool;

use crate::{Error, ProjectMarketUuid, ProjectUuid, Result};

pub async fn delete(
    pool:         &PgPool,
    project_uuid: ProjectUuid,
    market_uuid:  ProjectMarketUuid,
) -> Result<()> {
    sqlx::query!("
            DELETE FROM project_market
            WHERE project_id = $1
              AND id = $2
        ",
            *project_uuid,
            *market_uuid,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::DeleteMarket(e, project_uuid, market_uuid))
}
