use sqlx::PgPool;
use starfoundry_lib_worker::Task;

use crate::metric::WorkerMetric;
use crate::error::{Error, Result};
use crate::tasks::WorkerEveGatewayTask;

/// Ensures that all necessary tasks are in the queue and new structures
/// are added into the rotation
pub async fn sync_task(
    pool: &PgPool,
    task: &mut Task<WorkerMetric, WorkerEveGatewayTask>,
) -> Result<()> {
    match sync_tasks(
        pool,
    ).await {
        Ok(_) => {},
        Err(e) => task.append_error(e.to_string()),
    };

    match sync_character_assets(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} character assets"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    match sync_character_blueprints(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} character blueprints"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    match sync_corporation_assets(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} corporation assets"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    match sync_corporation_blueprints(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} corporation blueprints"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    Ok(())
}

/// Ensures that all necessary tasks are in the queue and new structures
/// are added into the rotation
pub async fn sync(
    pool: &PgPool,
) -> Result<()> {
    sync_tasks(
        pool,
    ).await?;

    sync_character_assets(
        pool,
    ).await?;
    sync_character_blueprints(
        pool,
    ).await?;

    sync_corporation_assets(
        pool,
    ).await?;
    sync_corporation_blueprints(
        pool,
    ).await?;

    Ok(())
}

async fn sync_tasks(
    pool: &PgPool,
) -> Result<()> {
    for task in vec![
        WorkerEveGatewayTask::Sync,
        WorkerEveGatewayTask::SystemIndex,
        WorkerEveGatewayTask::SystemIndexCompress,
    ] {
        let task: String = task.into();

        let task_db = sqlx::query!("
                SELECT 1 as exists
                FROM worker_queue
                WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
                AND task = $1
            ",
                task,
            )
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if task_db.is_none() {
            sqlx::query!("
                INSERT INTO worker_queue (task)
                SELECT $1
            ",
                task,
            )
            .execute(pool)
            .await
            .map_err(Error::SyncError)?;
        }
    }

    Ok(())
}

async fn sync_character_assets(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerEveGatewayTask::CharacterAssets.into();
    let entries = sqlx::query!("
            SELECT
                ec.character_id,
                c.corporation_id,
                ec.domain
            FROM eve_credential ec
            JOIN character c ON c.character_id = ec.character_id
            WHERE
                scopes && $1::VARCHAR[] AND
                character_main IS NULL
        ",
            &vec!["esi-assets.read_assets.v1".into()],
        )
        .fetch_all(pool)
        .await
        .map_err(Error::GenericSqlxError)?;

    let tasks = sqlx::query!("
            SELECT
                (additional_data ->> 'character_id')::INTEGER AS character_id,
                (additional_data ->> 'corporation_id')::INTEGER AS corporation_id,
                (additional_data ->> 'source')::VARCHAR AS source
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        ",
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    let mut new_entries = Vec::new();
    for entry in entries {
        if let None = tasks
            .iter()
            .find(|x| {
                x.character_id == Some(entry.character_id) &&
                x.source == Some(entry.domain.clone())
            }) {
                let additional_data = serde_json::json!({
                    "character_id": entry.character_id,
                    "corporation_id": entry.corporation_id,
                    "source": entry.domain,
                });
                new_entries.push(additional_data);
            }
    }

    tracing::info!("Added {} new character assets jobs", new_entries.len());
    sqlx::query!("
            INSERT INTO worker_queue (task, additional_data)
            SELECT $1, * FROM UNNEST(
                $2::JSONB[]
            )
        ",
            &task_name,
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::SyncError)
}

async fn sync_corporation_assets(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerEveGatewayTask::CorporationAssets.into();

    let entries = sqlx::query!("
            SELECT
                ec.character_id,
                c.corporation_id,
                ec.domain
            FROM eve_credential ec
            JOIN character c ON c.character_id = ec.character_id
            WHERE
                scopes && $1::VARCHAR[]
        ",
            &vec!["esi-assets.read_corporation_assets.v1".into()],
        )
        .fetch_all(pool)
        .await
        .map_err(Error::GenericSqlxError)?;

    let tasks = sqlx::query!("
            SELECT
                (additional_data ->> 'character_id')::INTEGER AS character_id,
                (additional_data ->> 'corporation_id')::INTEGER AS corporation_id,
                (additional_data ->> 'source')::VARCHAR AS source
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        ",
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    let mut new_entries = Vec::new();
    for entry in entries {
        if let None = tasks
            .iter()
            .find(|x| {
                x.corporation_id == Some(entry.corporation_id) &&
                x.source == Some(entry.domain.clone())
            }) {
                let additional_data = serde_json::json!({
                    "character_id": entry.character_id,
                    "corporation_id": entry.corporation_id,
                    "source": entry.domain,
                });
                new_entries.push(additional_data);
            }
    }

    tracing::info!("Added {} new corporation assets jobs", new_entries.len());
    sqlx::query!("
            INSERT INTO worker_queue (task, additional_data)
            SELECT $1, * FROM UNNEST(
                $2::JSONB[]
            )
        ",
            &task_name,
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::SyncError)
}

async fn sync_character_blueprints(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerEveGatewayTask::CharacterBlueprints.into();
    let entries = sqlx::query!("
            SELECT
                ec.character_id,
                c.corporation_id,
                ec.domain
            FROM eve_credential ec
            JOIN character c ON c.character_id = ec.character_id
            WHERE
                scopes && $1::VARCHAR[] AND
                character_main IS NULL
        ",
            &vec!["esi-characters.read_blueprints.v1".into()],
        )
        .fetch_all(pool)
        .await
        .map_err(Error::GenericSqlxError)?;

    let tasks = sqlx::query!("
            SELECT
                (additional_data ->> 'character_id')::INTEGER AS character_id,
                (additional_data ->> 'corporation_id')::INTEGER AS corporation_id,
                (additional_data ->> 'source')::VARCHAR AS source
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        ",
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    let mut new_entries = Vec::new();
    for entry in entries {
        if let None = tasks
            .iter()
            .find(|x| {
                x.character_id == Some(entry.character_id) &&
                x.source == Some(entry.domain.clone())
            }) {
                let additional_data = serde_json::json!({
                    "character_id": entry.character_id,
                    "corporation_id": entry.corporation_id,
                    "source": entry.domain,
                });
                new_entries.push(additional_data);
            }
    }

    tracing::info!("Added {} new character blueprint jobs", new_entries.len());
    sqlx::query!("
            INSERT INTO worker_queue (task, additional_data)
            SELECT $1, * FROM UNNEST(
                $2::JSONB[]
            )
        ",
            &task_name,
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::SyncError)
}

async fn sync_corporation_blueprints(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerEveGatewayTask::CorporationAssets.into();

    let entries = sqlx::query!("
            SELECT
                ec.character_id,
                c.corporation_id,
                ec.domain
            FROM eve_credential ec
            JOIN character c ON c.character_id = ec.character_id
            WHERE
                scopes && $1::VARCHAR[]
        ",
            &vec!["esi-corporation.read_blueprints.v1".into()],
        )
        .fetch_all(pool)
        .await
        .map_err(Error::GenericSqlxError)?;

    let tasks = sqlx::query!("
            SELECT
                (additional_data ->> 'character_id')::INTEGER AS character_id,
                (additional_data ->> 'corporation_id')::INTEGER AS corporation_id,
                (additional_data ->> 'source')::VARCHAR AS source
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        ",
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    let mut new_entries = Vec::new();
    for entry in entries {
        if let None = tasks
            .iter()
            .find(|x| {
                x.corporation_id == Some(entry.corporation_id) &&
                x.character_id == Some(entry.character_id) &&
                x.source == Some(entry.domain.clone())
            }) {
                let additional_data = serde_json::json!({
                    "character_id": entry.character_id,
                    "corporation_id": entry.corporation_id,
                    "source": entry.domain,
                });
                new_entries.push(additional_data);
            }
    }

    tracing::info!("Added {} new corporation blueprint jobs", new_entries.len());
    sqlx::query!("
            INSERT INTO worker_queue (task, additional_data)
            SELECT $1, * FROM UNNEST(
                $2::JSONB[]
            )
        ",
            &task_name,
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::SyncError)
}
