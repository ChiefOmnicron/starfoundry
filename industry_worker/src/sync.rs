use sqlx::PgPool;
use starfoundry_lib_worker::Task;

use crate::error::{Error, Result};
use crate::metric::WorkerMetric;
use crate::WorkerIndustryTask;

const SOURCE: &str = "industry.dev.starfoundry.space";

/// Ensures that all necessary tasks are in the queue and new structures
/// are added into the rotation
pub async fn sync_task(
    pool:   &PgPool,
    task:   &mut Task<WorkerMetric, WorkerIndustryTask>,
) -> Result<()> {
    //match sync_character_jobs(
    //    pool,
    //).await {
    //    Ok(new_entries) => {
    //        if new_entries > 0 {
    //            task.append_log(format!("added {new_entries} character jobs"))
    //        }
    //    },
    //    Err(e) => task.append_error(e.to_string()),
    //};

    match sync_corporation_jobs(
        pool,
    ).await {
        Ok(new_entries) => {
            if new_entries > 0 {
                task.append_log(format!("added {new_entries} corporation jobs"))
            }
        },
        Err(e) => task.append_error(e.to_string()),
    };

    Ok(())
}

/// Ensures that all necessary tasks are in the queue and new structures
/// are added into the rotation
pub async fn sync(
    pool:   &PgPool,
) -> Result<()> {
    //sync_character_jobs(
    //    pool,
    //).await?;

    sync_corporation_jobs(
        pool,
    ).await?;

    Ok(())
}

async fn sync_character_jobs(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerIndustryTask::JobCharacter.into();

    // FIXME: only tmp
    let characters = vec![];

    let market_stations = sqlx::query!("
            SELECT
                (additional_data ->> 'character_id')::BIGINT AS character_id
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        ",
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    // ensure that non authed structures are in the queue
    let mut new_entries = Vec::new();
    for character in characters {
        if let None = market_stations
            .iter()
            .find(|x| {
                x.character_id == Some(character)
            }) {
                let additional_data = serde_json::json!({
                    "character_id": character,
                    "source": SOURCE,
                });
                new_entries.push(additional_data);
            }
    }

    tracing::info!("Added {} new job character jobs", new_entries.len());
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

async fn sync_corporation_jobs(
    pool: &PgPool,
) -> Result<usize> {
    let task_name: String = WorkerIndustryTask::JobCorporation.into();

    // FIXME: only tmp
    // (corporation_id, character_id, main_character_id)
    let corporations = vec![
        // Flanders
        (98748294, 2117848811, 2117441999),
        // RCI
        //(98024275, 2117441999),
    ];

    let market_stations = sqlx::query!(r#"
            SELECT
                (additional_data ->> 'corporation_id')::BIGINT AS "corporation_id!",
                (additional_data ->> 'character_id')::INTEGER AS "character_id!"
            FROM worker_queue
            WHERE (status = 'WAITING' OR status = 'IN_PROGRESS')
            AND task = $1
        "#,
            &task_name,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::SyncError)?;

    // if there is at least one entry, skip it
    if !market_stations.is_empty() {
        return Ok(0usize);
    }

    // ensure that non authed structures are in the queue
    let mut new_entries = Vec::new();
    for (corporation, character, main_character) in corporations {
        if let None = market_stations
            .iter()
            .find(|x| {
                x.corporation_id == corporation &&
                x.character_id == character
            }) {
                let additional_data = serde_json::json!({
                    "corporation_id": corporation,
                    "character_id": character,
                    "main_character_id": main_character,
                    "source": SOURCE,
                });
                new_entries.push(additional_data);
            }
    }

    tracing::info!("Added {} new job corporation jobs", new_entries.len());
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
