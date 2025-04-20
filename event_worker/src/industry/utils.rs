use sqlx::PgPool;
use starfoundry_libs_eve_api::{EveApiClient, IndustryJobEntry};
use starfoundry_libs_projects::ProjectJobStatus;
use starfoundry_libs_types::{CharacterId, CorporationId, ItemId, JobId, LocationId};
use std::collections::HashMap;
use uuid::Uuid;

use crate::error::{Error, Result};
use super::{StartableIndustryJobs, UpdateJobRequest};

/// Fetches both the default group and the specific group jobs and joins them
/// together.
/// The jobs are sorted by the date the project was created
/// 
pub async fn fetch_startable_jobs(
    pool:          &PgPool,
    character_ids: Vec<CharacterId>,
) -> Result<Vec<StartableIndustryJobs>> {
    let mut default_group = fetch_startable_default_group_jobs(
            pool,
            character_ids.clone(),
        )
        .await?;

    let specific_group = fetch_startable_specific_group_jobs(
            pool,
            character_ids,
        )
        .await?;

    default_group.extend(specific_group);
    default_group.sort_by_key(|x| x.created_at);

    Ok(default_group)
}

/// Fetches all startable jobs, were the project is in the default group
/// 
pub async fn fetch_startable_default_group_jobs(
    pool:          &PgPool,
    character_ids: Vec<CharacterId>,
) -> Result<Vec<StartableIndustryJobs>> {
    sqlx::query_as!(
            StartableIndustryJobs,
            r#"
                SELECT
                    p.name AS "project_name",
                    project_id,
                    pj.id,
                    type_id,
                    runs,
                    pj.status AS "status!: ProjectJobStatus",
                    job_id AS "job_id: JobId",
                    created_at
                FROM project_jobs pj
                JOIN projects p ON p.id = pj.project_id
                WHERE p.status = 'IN_PROGRESS'
                AND (
                    pj.status = 'WAITING_FOR_MATERIALS' OR
                    pj.status = 'BUILDING'
                )
                AND p.owner = ANY($1)
                AND p.project_group_id = '00000000-0000-0000-0000-000000000000'
                ORDER BY p.created_at ASC
            "#,
            &character_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(Error::FetchActiveJobs)
}

/// Fetches all startable jobs, were a project is in a user created group
/// 
pub async fn fetch_startable_specific_group_jobs(
    pool:          &PgPool,
    character_ids: Vec<CharacterId>,
) -> Result<Vec<StartableIndustryJobs>> {
    sqlx::query_as!(
            StartableIndustryJobs,
            r#"
                SELECT
                    p.name AS "project_name",
                    project_id,
                    pj.id,
                    type_id,
                    runs,
                    pj.status AS "status!: ProjectJobStatus",
                    job_id AS "job_id: JobId",
                    created_at
                FROM project_jobs pj
                JOIN projects p ON p.id = pj.project_id
                JOIN project_group_members pgm ON pgm.group_id = p.project_group_id
                WHERE p.status = 'IN_PROGRESS'
                AND (
                    pj.status = 'WAITING_FOR_MATERIALS' OR
                    pj.status = 'BUILDING'
                )
                AND pgm.character_id = ANY($1)
                ORDER BY p.created_at ASC
            "#,
            &character_ids.into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(Error::FetchActiveJobs)
}

pub async fn fetch_done_job_ids(
    pool:    &PgPool,
    job_ids: Vec<i32>,
) -> Result<Vec<JobId>> {
    sqlx::query!(r#"
            SELECT job_id AS "job_id!"
            FROM project_jobs
            WHERE job_id = ANY($1)
            AND status = 'DONE'
        "#,
            &job_ids,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::FetchDoneJobIds)
        .map(|x| {
            x.into_iter()
                .map(|x| x.job_id.into())
                .collect::<Vec<_>>()
        })
}

pub async fn resolve_container_names(
    _pool:               &PgPool,
    client:              &EveApiClient,
    location_ids:        &Vec<LocationId>,
    output_location_ids: &Vec<LocationId>,
) -> Result<HashMap<i64, String>> {
    let mut containers = HashMap::new();

    let mut item_ids = output_location_ids
        .iter()
        .filter(|x| !location_ids.contains(&x))
        .map(|x| ItemId(**x))
        .collect::<Vec<_>>();
    item_ids.sort();
    item_ids.dedup();

    for item_id in item_ids {
        let name = match client
            .asset_names(vec![item_id])
            .await {

            // # is an accepted special character, why? because I say so
            Ok(x)  => x.into_iter().map(|x| x.name.replace("#", "")).collect::<Vec<_>>(),
            Err(_) => {
                // ignore errors
                continue;
            }
        };
        let name = name.first().unwrap();
        containers.insert(*item_id, name.clone());
    }

    Ok(containers)
}

pub async fn resolve_main_character_from_character(
    pool:         &PgPool,
    character_id: CharacterId,
) -> Result<CharacterId> {
    sqlx::query!("
            SELECT character_main
            FROM credentials
            WHERE character_id = $1
        ",
            *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::FetchMainCharacterByCharacter(e, character_id))
        .map(|x| {
            if let Some(x) = x {
                if let Some(x) = x.character_main {
                    x.into()
                } else {
                    character_id
                }
            } else {
                character_id
            }
        })
}

pub async fn resolve_main_character_from_corporation(
    pool:           &PgPool,
    corporation_id: CorporationId,
) -> Result<Vec<CharacterId>> {
    sqlx::query!("
            SELECT character_main
            FROM credentials
            WHERE character_id = $1
        ",
            *corporation_id,
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::FetchMainCharacterByCorporation(e, corporation_id))
        .map(|x| {
            x.into_iter()
                .filter(|x| x.character_main.is_some())
                // unwrap is safe, as we remove all none values and corporations
                // will always have the character_main field set
                .map(|x| x.character_main.unwrap().into())
                .collect::<Vec<_>>()
        })
}

pub async fn update_industry_jobs(
    pool:    &PgPool,
    updates: &HashMap<Uuid, Vec<UpdateJobRequest>>,
) -> Result<()> {
    if updates.is_empty() {
        return Ok(());
    }

    for (project_id, updates) in updates {
        let ids = updates
            .iter()
            .map(|x| x.id)
            .collect::<Vec<_>>();
        let costs = updates
            .iter()
            .map(|x| x.cost)
            .collect::<Vec<_>>();
        let status = updates
            .iter()
            .map(|x| x.status.clone())
            .collect::<Vec<_>>();
        let job_ids = updates
            .iter()
            .map(|x| x.job_id.clone())
            .collect::<Vec<_>>();
        let character_ids = updates
            .iter()
            .map(|x| x.character_id.map(|x| *x))
            .collect::<Vec<_>>();

        sqlx::query!("
                UPDATE project_jobs
                SET cost   =       data.cost,
                    status =       data.status,
                    job_id =       data.job_id,
                    character_id = data.character_id
                FROM (
                    SELECT
                        UNNEST($2::UUID[]) AS id,
                        UNNEST($3::REAL[]) AS cost,
                        UNNEST($4::PROJECT_JOB_STATUS[]) AS status,
                        UNNEST($5::INTEGER[]) AS job_id,
                        UNNEST($6::INTEGER[]) AS character_id
                ) AS data
                WHERE project_id = $1
                AND project_jobs.id = data.id
            ",
                project_id,
                &ids,
                &costs as _,
                &status as _,
                &job_ids as _,
                &character_ids as _,
            )
            .execute(pool)
            .await
            .map(drop)
            .map_err(Error::UpdateCorporationJobEntry)?;

        // TODO: implement a better solution
        // resets entries that don't have a cost, but a job id
        for entry in updates {
            if entry.cost.is_none() && entry.job_id.is_some() {
                sqlx::query!("
                        UPDATE project_jobs
                        SET job_id = NULL
                        WHERE id = $1
                    ",
                        entry.id,
                    )
                    .execute(pool)
                    .await
                    .unwrap();
            }
        }
    }

    Ok(())
}

pub async fn update_finished_jobs(
    pool: &PgPool,
) -> Result<()> {
    sqlx::query!("
            UPDATE project_jobs
            SET status = 'DONE'
            WHERE status = 'BUILDING'
            AND id = ANY(
                SELECT id
                FROM project_jobs pj
                JOIN industry_jobs ij ON ij.job_id = pj.job_id
                WHERE pj.status = 'BUILDING'
                AND TO_TIMESTAMP(end_date, 'YYYY-MM-DDTHH:MI:SS') < NOW()
                AND ij.is_delivered = true
            )
        ")
        .execute(pool)
        .await
        .map(drop)
        .map_err(Error::UpdateAlreadyDoneJobs)
}

pub async fn insert_job_detection_log(
    pool:     &PgPool,
    updates:  &HashMap<Uuid, Vec<UpdateJobRequest>>,
    umatched: &Vec<IndustryJobEntry>,
) -> Result<()> {
    let mut type_ids    = Vec::new();
    let mut job_ids     = Vec::new();
    let mut project_ids = Vec::new();

    let updates = updates
        .into_iter()
        .map(|(_, x)| x)
        .flatten()
        .collect::<Vec<_>>();

    for entry in updates {
        type_ids.push(entry.type_id);
        // if it was matched, it will always have a job_id
        job_ids.push(entry.job_id.unwrap_or_default());
        project_ids.push(entry.project_id);
    }

    for entry in umatched {
        type_ids.push(entry.product_type_id);
        job_ids.push(*entry.job_id);
        project_ids.push(None);
    }

    sqlx::query!("
            INSERT INTO job_detection_logs
            (
                type_id,
                job_id,
                project_id
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::UUID[]
            )
            ON CONFLICT (job_id)
            DO NOTHING
        ",
            &type_ids as _,
            &job_ids,
            &project_ids as _,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(Error::UpdateCorporationJobEntry)
}

pub async fn cleanup_delivered_jobs(
    pool:                     &PgPool,
    //character_corporation_id: i32,
) -> Result<()> {
    sqlx::query!("
            UPDATE industry_jobs
            SET is_delivered = true
            WHERE is_delivered = false
            AND end_date::TIMESTAMPTZ < NOW()
            RETURNING job_id
        ")
        .fetch_all(pool)
        .await
        .map(drop)
        .map_err(Error::UpdateDeliveredJobs)
}

pub async fn fetch_ignored_jobs(
    pool: &PgPool,
) -> Result<Vec<JobId>> {
    let ignored_jobs = sqlx::query!("
            SELECT job_id
            FROM industry_jobs
            WHERE is_delivered = false
            AND ignore = true
        ")
        .fetch_all(pool)
        .await
        .unwrap()
        .into_iter()
        .map(|x| x.job_id.into())
        .collect::<Vec<_>>();
    Ok(ignored_jobs)
}
