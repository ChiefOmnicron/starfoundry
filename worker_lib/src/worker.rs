use sqlx::{types::Uuid, PgPool};
use std::time::Duration;
use tokio::time::sleep;

use crate::error::{Error, Result};

/// Registers itself as a new worker
/// 
pub async fn register_worker(
    pool: &PgPool,
) -> Result<Uuid> {
    let worker_id = sqlx::query!("
            INSERT INTO worker_registry (last_seen)
            VALUES (NOW())
            RETURNING id
        ")
        .fetch_one(pool)
        .await
        .map_err(Error::RegisterWorker)?
        .id;

    if let Err(e) = remove_dead_workers(pool).await {
        tracing::warn!("failed deleting dead workers, {e}");
    }

    tracing::info!("worker registered");

    Ok(worker_id)
}

/// Registers itself, removes dead workers and refreshing it's last seen timer
/// 
pub async fn background_task(
    pool:      PgPool,
    worker_id: Uuid
) -> Result<()> {
    loop {
        remove_dead_workers(&pool).await?;
        remove_stalled_jobs(&pool).await?;
        refresh_last_seen(&pool, worker_id).await?;

        // sleep for 30 seconds until the next refresh
        sleep(Duration::from_secs(30)).await;
    }
}


/// Updates the last seen timer for a worker
/// 
async fn refresh_last_seen(
    pool:      &PgPool,
    worker_id: Uuid,
) -> Result<()> {
    sqlx::query!("
            UPDATE worker_registry
            SET last_seen = NOW()
            WHERE id = $1
        ",
            worker_id,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(|e| Error::UpdateWorkerLastSeen(e, worker_id))
}

/// Deletes all workers that haven't updated their `last_seen` field
/// in the last 5 minutes
/// 
async fn remove_dead_workers(
    pool: &PgPool,
) -> Result<()> {
    let dead_worker_id = sqlx::query!("
            DELETE FROM worker_registry
            WHERE id IN (
                SELECT id
                FROM worker_registry
                WHERE last_seen < NOW() - INTERVAL '5 minutes'
                LIMIT 1
            )
            RETURNING id
        ")
        .fetch_optional(pool)
        .await
        .map_err(Error::DeleteDeadWorker)?;

    if let Some(dead_worker_id) = dead_worker_id {
        let dead_worker_id = dead_worker_id.id;
        sqlx::query!("
            UPDATE worker_queue
            SET
                worker_id = NULL,
                status = 'WAITING'
            WHERE worker_id = $1
            AND status = 'IN_PROGRESS'
        ",
            dead_worker_id,
        )
        .execute(pool)
        .await
        .map_err(Error::UpdateTaskFromDeadWorker)?;
    }

    Ok(())
}

/// Removes all jobs that are still in progress after 15 minutes
/// TODO: the service needs a self check and kill itself if it's stalled
/// 
async fn remove_stalled_jobs(
    pool: &PgPool,
) -> Result<()> {
    sqlx::query!("
            UPDATE worker_queue
            SET
                status = 'WAITING',
                worker_id = NULL
            WHERE id IN (
                SELECT id
                FROM worker_queue
                WHERE started_at < NOW() - INTERVAL '15 minutes'
                AND status = 'IN_PROGRESS'
                LIMIT 1
            )
            RETURNING id
        ")
        .fetch_optional(pool)
        .await
        .map_err(Error::DeleteDeadWorker)?;

    Ok(())
}
