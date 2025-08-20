use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    task: &mut Task,
    pool: &PgPool,
) -> Result<()> {
    match cleanup_self(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added self cleanup"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    match appraisals(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added cleanup industry index"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    match industry_index(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added cleanup industry index"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    Ok(())
}

async fn cleanup_self(
    pool: &PgPool
) -> Result<usize> {
    let has_entry = sqlx::query!("
            SELECT COUNT(1) AS count
            FROM event_queue
            WHERE task = 'CLEANUP_SELF'
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
        VALUES ('CLEANUP_SELF')
    ")
    .execute(pool)
    .await
    .map(|_| 1)
    .map_err(Error::InsertNewJobs)
}

async fn appraisals(
    pool: &PgPool
) -> Result<usize> {
    let has_entry = sqlx::query!("
            SELECT COUNT(1) AS count
            FROM event_queue
            WHERE task = 'CLEANUP_APPRAISALS'
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
        VALUES ('CLEANUP_APPRAISALS')
    ")
    .execute(pool)
    .await
    .map(|_| 1)
    .map_err(Error::InsertNewJobs)
}

async fn industry_index(
    pool: &PgPool
) -> Result<usize> {
    let has_entry = sqlx::query!("
            SELECT COUNT(1) AS count
            FROM event_queue
            WHERE task = 'CLEANUP_INDUSTRY_INDEX'
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
        VALUES ('CLEANUP_INDUSTRY_INDEX')
    ")
    .execute(pool)
    .await
    .map(|_| 1)
    .map_err(Error::InsertNewJobs)
}
