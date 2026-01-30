use sqlx::{PgPool, Postgres};

use crate::error::{Error, Result};
use crate::task::Task;
use crate::{TaskMetric, WorkerTask};

pub async fn cleanup_task<M, WT>(
    pool:  &PgPool,
    _task: &mut Task<M, WT>,
) -> Result<()>
    where
        M: TaskMetric,
        WT: WorkerTask,
        WT: std::fmt::Debug + sqlx::Type<Postgres> + Into<String>,
        <WT as TryFrom<String>>::Error: std::fmt::Debug {
        sqlx::query!("
                DELETE FROM worker_queue
                WHERE DATE(process_after) < DATE(NOW() - INTERVAL '3 DAY')
            ")
            .execute(pool)
            .await
            .map_err(Error::CleanupOldTasks)?;
    Ok(())
}
