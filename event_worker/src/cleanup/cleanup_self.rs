use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    _tasl: &mut Task,
    pool:  &PgPool,
) -> Result<()> {
        sqlx::query!("
                DELETE FROM event_queue
                WHERE DATE(process_after) < DATE(NOW() - INTERVAL '3 DAY')
            ")
            .execute(pool)
            .await
            .map_err(Error::DeleteRedundantEventQueue)?;
    Ok(())
}
