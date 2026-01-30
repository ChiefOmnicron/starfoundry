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
    let mut new_tasks = 0;

    let entries = sqlx::query!("
            SELECT character_id, domain
            FROM eve_credential
            WHERE
                scopes && $1::VARCHAR[] AND
                character_main IS NULL
        ",
            &vec!["esi-assets.read_assets.v1".into()],
        )
        .fetch_all(pool)
        .await
        .map_err(Error::GenericSqlxError)?;

    for entry in entries {
        let task = sqlx::query!("
                SELECT
                    (additional_data ->> 'character_id')::INTEGER AS character_id,
                    (additional_data ->> 'source')::VARCHAR AS source
                FROM worker_queue
                WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
                AND task = $1
            ",
                &task_name,
            )
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if task.is_none() {
            new_tasks += 1;

            let result = sqlx::query!("
                    INSERT INTO worker_queue (task, additional_data)
                    SELECT $1, $2
                ",
                    &task_name,
                    &serde_json::json!({
                        "character_id": entry.character_id,
                        "source": entry.domain,
                    })
                )
                .execute(pool)
                .await
                .map_err(Error::SyncError);

            if let Err(e) = result {
                tracing::error!("{:?}", e);
            }
        }
    }

    Ok(new_tasks)
}

async fn sync_corporation_assets(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerEveGatewayTask::CorporationAssets.into();
    let mut new_tasks = 0;

    let entries = sqlx::query!("
            SELECT character_id, character_main, domain
            FROM eve_credential
            WHERE
                scopes && $1::VARCHAR[] AND
                character_main IS NOT NULL
        ",
            &vec!["esi-assets.read_corporation_assets.v1".into()],
        )
        .fetch_all(pool)
        .await
        .map_err(Error::GenericSqlxError)?;

    for entry in entries {
        let task = sqlx::query!("
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
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if task.is_none() {
            new_tasks += 1;

            let result = sqlx::query!("
                    INSERT INTO worker_queue (task, additional_data)
                    SELECT $1, $2
                ",
                    &task_name,
                    &serde_json::json!({
                        "character_id": entry.character_main,
                        "corporation_id": entry.character_id,
                        "source": entry.domain,
                    })
                )
                .execute(pool)
                .await
                .map_err(Error::SyncError);

            if let Err(e) = result {
                tracing::error!("{:?}", e);
            }
        }
    }

    Ok(new_tasks)
}

async fn sync_character_blueprints(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerEveGatewayTask::CharacterBlueprints.into();
    let mut new_tasks = 0;

    let entries = sqlx::query!("
            SELECT character_id, domain
            FROM eve_credential
            WHERE
                scopes && $1::VARCHAR[] AND
                character_main IS NULL
        ",
            &vec!["esi-characters.read_blueprints.v1".into()],
        )
        .fetch_all(pool)
        .await
        .map_err(Error::GenericSqlxError)?;

    for entry in entries {
        let task = sqlx::query!("
                SELECT
                    (additional_data ->> 'character_id')::INTEGER AS character_id,
                    (additional_data ->> 'source')::VARCHAR AS source
                FROM worker_queue
                WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
                AND task = $1
            ",
                &task_name,
            )
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if task.is_none() {
            new_tasks += 1;

            let result = sqlx::query!("
                    INSERT INTO worker_queue (task, additional_data)
                    SELECT $1, $2
                ",
                    &task_name,
                    &serde_json::json!({
                        "character_id": entry.character_id,
                        "source": entry.domain,
                    })
                )
                .execute(pool)
                .await
                .map_err(Error::SyncError);

            if let Err(e) = result {
                tracing::error!("{:?}", e);
            }
        }
    }

    Ok(new_tasks)
}

async fn sync_corporation_blueprints(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerEveGatewayTask::CorporationBlueprints.into();
    let mut new_tasks = 0;

    let entries = sqlx::query!("
            SELECT character_id, character_main, domain
            FROM eve_credential
            WHERE
                scopes && $1::VARCHAR[] AND
                character_main IS NOT NULL
        ",
            &vec!["esi-corporations.read_blueprints.v1".into()],
        )
        .fetch_all(pool)
        .await
        .map_err(Error::GenericSqlxError)?;

    for entry in entries {
        let task = sqlx::query!("
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
            .fetch_optional(pool)
            .await
            .map_err(Error::SyncError)?;

        if task.is_none() {
            new_tasks += 1;

            let result = sqlx::query!("
                    INSERT INTO worker_queue (task, additional_data)
                    SELECT $1, $2
                ",
                    &task_name,
                    &serde_json::json!({
                        "character_id": entry.character_main,
                        "corporation_id": entry.character_id,
                        "source": entry.domain,
                    })
                )
                .execute(pool)
                .await
                .map_err(Error::SyncError);

            if let Err(e) = result {
                tracing::error!("{:?}", e);
            }
        }
    }

    Ok(new_tasks)
}
