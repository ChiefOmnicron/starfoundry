use sqlx::PgPool;
use sqlx::types::chrono::NaiveDateTime;
use starfoundry_libs_structures::StructureUuid;

use crate::{Error, Result};

pub async fn last_fetch(
    pool:           &PgPool,
    structure_uuid: StructureUuid,
) -> Result<Option<NaiveDateTime>> {
    let entry = sqlx::query!("
            SELECT finished_at
            FROM event_queue
            WHERE (additional_data ->> 'structure_id')::BIGINT = (
                SELECT structure_id
                FROM structure
                WHERE id = $1
            )
            AND status = 'DONE'
            ORDER BY finished_at DESC
        ",
            *structure_uuid,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::LatestMarketFetch(e, structure_uuid))?
        .ok_or(Error::NoLatestMarketFetch)?;

    Ok(entry.finished_at)
}
