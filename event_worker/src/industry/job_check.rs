use sqlx::PgPool;

use crate::error::{Error, Result};
use crate::task::Task;

pub async fn task(
    task: &mut Task,
    pool: &PgPool,
) -> Result<()> {
    match character_jobs(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added {new_entries} character jobs"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    match corporation_jobs(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added {new_entries} corporation jobs"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    match industry_index(pool).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.add_log(format!("Added industry index"))
            }
        },
        Err(e) => task.add_error(e.to_string()),
    };

    Ok(())
}

async fn character_jobs(
    pool: &PgPool
) -> Result<usize> {
    let mut character_ids_target = sqlx::query!("
            SELECT c.character_id
            FROM character c
            JOIN credential cc ON cc.character_id = c.character_id
            WHERE c.character_id != 0
        ")
        .fetch_all(pool)
        .await
        .map_err(Error::FetchCharacterIds)
        .map(|x| {
            x
                .iter()
                .map(|y| y.character_id)
                .collect::<Vec<_>>()
        })?;

    let character_ids_is = sqlx::query!("
            SELECT (additional_data ->> 'character_id')::INTEGER AS character_id
            FROM event_queue
            WHERE (additional_data ->> 'character_id')::INTEGER = ANY($1)
            AND (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = 'INDUSTRY_JOBS_CHARACTER'
        ",
            &character_ids_target,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::FetchCharacterIdsQueue)
        .map(|x| {
            x
                .iter()
                .map(|y| y.character_id.unwrap_or_default())
                .collect::<Vec<_>>()
        })?;

    if character_ids_is.len() == character_ids_target.len() {
        return Ok(0);
    }

    character_ids_target.sort();
    character_ids_target.dedup();
    let mut new_entries = Vec::new();
    for character_id in character_ids_target {
        if !character_ids_is.contains(&character_id) {
            let additional_data = serde_json::json!({
                "character_id": character_id,
            });
            new_entries.push(additional_data);
        }
    }

    tracing::info!("Added {} new character jobs", new_entries.len());

    sqlx::query!("
            INSERT INTO event_queue (task, additional_data)
            SELECT 'INDUSTRY_JOBS_CHARACTER', * FROM UNNEST(
                $1::JSONB[]
            )
        ",
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::InsertNewJobs)
}

async fn corporation_jobs(
    pool: &PgPool
) -> Result<usize> {
    let mut corporation_target = sqlx::query!(r#"
            SELECT character_id AS "character_id!"
            FROM credential
            WHERE credential_type = 'CORPORATION'
        "#)
        .fetch_all(pool)
        .await
        .map_err(Error::FetchCharacterIds)
        .map(|x| {
            x
                .iter()
                .map(|y| y.character_id)
                .collect::<Vec<_>>()
        })?;

    let corporation_is = sqlx::query!("
            SELECT (additional_data ->> 'corporation_id')::INTEGER AS corporation_id
            FROM event_queue
            WHERE (additional_data ->> 'corporation_id')::INTEGER = ANY($1)
            AND (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = 'INDUSTRY_JOBS_CORPORATION'
        ",
            &corporation_target,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::FetchCharacterIdsQueue)
        .map(|x| {
            x
                .iter()
                .map(|y| y.corporation_id.unwrap_or_default())
                .collect::<Vec<_>>()
        })?;

    corporation_target.sort();
    corporation_target.dedup();
    let mut new_entries = Vec::new();
    for corporation_id in corporation_target {
        if !corporation_is.contains(&corporation_id) {
            let additional_data = serde_json::json!({
                "corporation_id": corporation_id,
            });
            new_entries.push(additional_data);
        }
    }

    tracing::info!("Added {} new corporation jobs", new_entries.len());

    sqlx::query!("
            INSERT INTO event_queue (task, additional_data)
            SELECT 'INDUSTRY_JOBS_CORPORATION', * FROM UNNEST(
                $1::JSONB[]
            )
        ",
            &new_entries
        )
        .execute(pool)
        .await
        .map(|_| new_entries.len())
        .map_err(Error::InsertNewJobs)
}

pub async fn industry_index(
    pool: &PgPool
) -> Result<usize> {
    let has_entry = sqlx::query!("
            SELECT COUNT(1) AS count
            FROM event_queue
            WHERE task = 'INDUSTRY_INDEX'
        ")
        .fetch_one(pool)
        .await
        .map(|x| x.count.unwrap_or_default() > 0)
        .map_err(Error::FetchTask)?;

    if has_entry {
        return Ok(0);
    }

    dbg!("no industry index entry, adding");
    sqlx::query!("
        INSERT INTO event_queue (task)
        VALUES ('INDUSTRY_INDEX')
    ")
    .execute(pool)
    .await
    .map(|_| 1)
    .map_err(Error::InsertNewJobs)
}
