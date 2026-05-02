use sqlx::PgPool;
use starfoundry_lib_industry::MarketUuid;
use starfoundry_lib_industry::ProjectUuid;

use crate::project::error::ProjectError;
use crate::project::error::Result;

pub async fn delete_market_entry(
    pool:           &PgPool,
    project_id:     ProjectUuid,
    market_id:      MarketUuid,
) -> Result<()> {
    sqlx::query!(r#"
            DELETE FROM project_market
            WHERE project_id = $1
            AND id = $2
        "#,
            *project_id,
            *market_id,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| ProjectError::Delete(e, project_id))
}
