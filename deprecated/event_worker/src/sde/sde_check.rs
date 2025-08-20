use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    task: &mut Task,
    pool: &PgPool,
) -> Result<()> {
    match sde_download(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added sde download"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    Ok(())
}

async fn sde_download(
    pool: &PgPool
) -> Result<usize> {
    let has_entry = sqlx::query!("
            SELECT COUNT(1) AS count
            FROM event_queue
            WHERE task = 'SDE_DOWNLOAD'
        ")
        .fetch_one(pool)
        .await
        .map(|x| x.count.unwrap_or_default() > 0)
        .map_err(Error::FetchTask)?;

    if has_entry {
        return Ok(0);
    }

    sqlx::query!("
        INSERT INTO event_queue (task)
        VALUES ('SDE_DOWNLOAD')
    ")
    .execute(pool)
    .await
    .map(|_| 1)
    .map_err(Error::InsertNewJobs)
}
