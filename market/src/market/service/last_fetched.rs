use starfoundry_lib_types::StructureId;
use sqlx::PgPool;
use chrono::NaiveDateTime;

use crate::market::error::{MarketError, Result};

pub async fn last_fetched(
    pool:           &PgPool,
    structure_id:   &StructureId,
) -> Result<NaiveDateTime> {
    let entry = sqlx::query!("
            SELECT finished_at
            FROM worker_queue
            WHERE (additional_data ->> 'structure_id')::BIGINT = $1
            AND (task = 'LATEST_NPC' OR task = 'LATEST_PLAYER')
            AND status = 'DONE'
            ORDER BY finished_at DESC
            LIMIT 1
        ",
            **structure_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(MarketError::LatestFetch)?
        .ok_or(MarketError::NotFound(*structure_id))?;

    if let Some(x) = entry.finished_at {
        Ok(x)
    } else {
        Err(MarketError::NotFound(*structure_id))
    }
}
