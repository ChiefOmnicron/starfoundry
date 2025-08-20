use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::WorkerTask;

pub async fn task(
    pool: &PgPool,
) -> Result<()> {
    match add_check_if_not_exists(pool, &WorkerTask::AssetCheck).await {
        Ok(true) => {
            tracing::info!("added new AssetCheck")
        },
        Ok(false) => (),
        Err(e) => tracing::error!("{}", e.to_string()),
    };

    match add_check_if_not_exists(pool, &WorkerTask::CleanupCheck).await {
        Ok(true) => {
            tracing::info!("added new CleanupCheck")
        },
        Ok(false) => (),
        Err(e) => tracing::error!("{}", e.to_string()),
    };

    match add_check_if_not_exists(pool, &WorkerTask::MarketCheck).await {
        Ok(true) => {
            tracing::info!("added new MarketCheck")
        },
        Ok(false) => (),
        Err(e) => tracing::error!("{}", e.to_string()),
    };

    match add_check_if_not_exists(pool, &WorkerTask::IndustryCheck).await {
        Ok(true) => {
            tracing::info!("added new IndustryCheck")
        },
        Ok(false) => (),
        Err(e) => tracing::error!("{}", e.to_string()),
    };

    match add_check_if_not_exists(pool, &WorkerTask::SdeCheck).await {
        Ok(true) => {
            tracing::info!("added new IndustryCheck")
        },
        Ok(false) => (),
        Err(e) => tracing::error!("{}", e.to_string()),
    };

    match add_check_if_not_exists(pool, &WorkerTask::StockCheck).await {
        Ok(true) => {
            tracing::info!("added new StockCheck")
        },
        Ok(false) => (),
        Err(e) => tracing::error!("{}", e.to_string()),
    };

    Ok(())
}

async fn add_check_if_not_exists(
    pool: &PgPool,
    task: &WorkerTask,
) -> Result<bool, Error> {
    let tasks = sqlx::query!("
            SELECT 1 AS count
            FROM event_queue
            WHERE task = $1
            AND (status = 'WAITING' OR status = 'IN_PROGRESS')
        ",
            task as _,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchCheck(e, *task))
        .map(|x| x.is_some())?;

    if !tasks {
        sqlx::query!("
                INSERT INTO event_queue
                VALUES($1)
            ",
                task as _,
            )
            .execute(pool)
            .await
            .map_err(|e| Error::InsertCheck(e, *task))?;
    }

    Ok(false)
}
