use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::EveGatewayApiClient;
use starfoundry_lib_types::{CharacterId, CorporationId, ItemId, JobId, TypeId};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{error::{Error, Result}, jobs::UnmatchedJob};

/// Fetches both the default group and the specific group jobs and joins them
/// together.
/// The jobs are sorted by the date the project was created
/// 
pub async fn fetch_startable_jobs(
    pool:           &PgPool,
    character_id:   CharacterId,
) -> Result<Vec<StartableIndustryJobs>> {
    // TODO: consider introducing the READY_TO_START flag
    sqlx::query_as!(
            StartableIndustryJobs,
            r#"
                SELECT
                    p.name AS "project_name",
                    project_id,
                    pj.id,
                    type_id,
                    runs,
                    pj.status AS "status!: ProjectJobStatusDatabase",
                    job_id AS "job_id: JobId",
                    pj.created_at
                FROM project_job pj
                JOIN project p ON p.id = pj.project_id
                JOIN project_group_member pgm ON pgm.project_group_id = p.project_group_id
                WHERE p.status = 'IN_PROGRESS'
                AND (
                    pj.status = 'WAITING_FOR_MATERIALS' OR
                    pj.status = 'BUILDING'
                )
                AND pgm.character_id = $1
                ORDER BY p.created_at ASC
            "#,
            *character_id,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::ListJobs)
}

pub async fn fetch_done_job_ids(
    pool:    &PgPool,
    job_ids: Vec<i32>,
) -> Result<Vec<JobId>> {
    sqlx::query!(r#"
            SELECT job_id AS "job_id!"
            FROM project_job
            WHERE job_id = ANY($1)
            AND status = 'DONE'
        "#,
            &job_ids,
        )
        .fetch_all(pool)
        .await
        .map_err(Error::ListJobs)
        .map(|x| {
            x.into_iter()
                .map(|x| x.job_id.into())
                .collect::<Vec<_>>()
        })
}

pub async fn resolve_corporation_asset_name(
    _pool:                  &PgPool,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    source:                 &String,
    character_id:           &CharacterId,
    corporation_id:         &CorporationId,
    location_ids:           &Vec<ItemId>,
    output_location_ids:    &Vec<ItemId>,
) -> Result<HashMap<ItemId, String>> {
    let mut containers = HashMap::new();

    let mut item_ids = output_location_ids
        .into_iter()
        .filter(|x| !location_ids.contains(&x))
        .collect::<Vec<_>>();
    item_ids.sort();
    item_ids.dedup();

    for item_id in item_ids {
        let result = match eve_gateway_api_client
                .eve_resolve_corporation_asset(
                    source,
                    character_id,
                    corporation_id,
                    vec![*item_id],
                )
                .await {

                Ok(x)  => x,
                Err(_) => {
                    // ignore errors
                    continue;
                }
            }
            .into_iter()
            .map(|x| (x.item_id, x.name))
            .collect::<HashMap<_, _>>();
        containers.extend(result);
    }

    Ok(containers)
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

        sqlx::query!("
                UPDATE project_job
                SET cost   =       data.cost,
                    status =       data.status,
                    job_id =       data.job_id
                FROM (
                    SELECT
                        UNNEST($2::UUID[]) AS id,
                        UNNEST($3::REAL[]) AS cost,
                        UNNEST($4::PROJECT_JOB_STATUS[]) AS status,
                        UNNEST($5::INTEGER[]) AS job_id
                ) AS data
                WHERE project_id = $1
                AND project_job.id = data.id
            ",
                project_id,
                &ids,
                &costs as _,
                &status as _,
                &job_ids as _,
            )
            .execute(pool)
            .await
            .map(drop)
            .map_err(Error::UpdateJob)?;

        // TODO: implement a better solution
        // resets entries that don't have a cost, but a job id
        for entry in updates {
            if entry.cost.is_none() && entry.job_id.is_some() {
                sqlx::query!("
                        UPDATE project_job
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
            UPDATE project_job
            SET status = 'DONE'
            WHERE status = 'BUILDING'
            AND id = ANY(
                SELECT id
                FROM project_job pj
                JOIN industry_job ij ON ij.job_id = pj.job_id
                WHERE pj.status = 'BUILDING'
                AND TO_TIMESTAMP(end_date, 'YYYY-MM-DDTHH:MI:SS') < NOW()
                AND ij.is_delivered = true
            )
        ")
        .execute(pool)
        .await
        .map(drop)
        .map_err(Error::UpdateJob)
}

pub async fn insert_job_detection_log(
    pool:           &PgPool,
    updates:        &HashMap<Uuid, Vec<UpdateJobRequest>>,
    unmatched_jobs: &Vec<UnmatchedJob>,
) -> Result<()> {
    let mut type_ids    = Vec::new();
    let mut job_ids     = Vec::new();
    let mut project_ids = Vec::new();
    let mut result      = Vec::new();

    let updates = updates
        .into_iter()
        .map(|(_, x)| x)
        .flatten()
        .collect::<Vec<_>>();

    for entry in updates {
        if job_ids.contains(&entry.job_id.unwrap_or_default()) {
            continue;
        }

        type_ids.push(entry.type_id);
        // if it was matched, it will always have a job_id
        job_ids.push(entry.job_id.unwrap_or_default());
        project_ids.push(entry.project_id);
        result.push("MATCHED".into());
    }

    for entry in unmatched_jobs {
        if job_ids.contains(&entry.job.job_id) {
            continue;
        }

        type_ids.push(entry.job.product_type_id);
        job_ids.push(*entry.job.job_id);
        project_ids.push(entry.project_id);
        result.push(entry.reason.into_string());
    }

    sqlx::query!("
            INSERT INTO job_detection_log
            (
                type_id,
                job_id,
                project_id,
                result
            )
            SELECT * FROM UNNEST(
                $1::INTEGER[],
                $2::INTEGER[],
                $3::UUID[],
                $4::VARCHAR[]
            )
            ON CONFLICT (job_id)
            DO UPDATE SET
                result = EXCLUDED.result
        ",
            &type_ids as _,
            &job_ids,
            &project_ids as _,
            &result as _,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(Error::InsertJobDetectionLog)
}

pub async fn cleanup_delivered_jobs(
    pool:                     &PgPool,
    //character_corporation_id: i32,
) -> Result<()> {
    sqlx::query!("
            UPDATE industry_job
            SET is_delivered = true
            WHERE is_delivered = false
            AND end_date::TIMESTAMPTZ < NOW()
            RETURNING job_id
        ")
        .fetch_all(pool)
        .await
        .map(drop)
        .map_err(Error::Cleanup)
}

#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type, Deserialize, Serialize, ToSchema,
)]
#[sqlx(type_name = "PROJECT_JOB_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectJobStatusDatabase {
    WaitingForMaterials,
    Building,
    Done,
}

#[derive(Clone, Debug, Serialize)]
pub struct StartableIndustryJobs {
    pub project_name: String,
    pub project_id:   Uuid,
    pub id:           Uuid,
    pub type_id:      TypeId,
    pub runs:         i32,
    pub status:       ProjectJobStatusDatabase,
    /// JobId from CCP
    pub job_id:       Option<JobId>,
    pub created_at:   DateTime<Utc>,
}

#[derive(Clone, Debug)]
pub struct UpdateJobRequest {
    pub id:             Uuid,
    pub character_id:   Option<CharacterId>,
    pub project_id:     Option<Uuid>,
    pub type_id:        TypeId,
    pub status:         ProjectJobStatusDatabase,
    pub cost:           Option<f32>,
    pub job_id:         Option<i32>,
}
