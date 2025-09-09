use serde::Deserialize;
use sqlx::PgPool;
use starfoundry_lib_eve_api::{Credentials, IndustryActivity};
use starfoundry_lib_types::CorporationId;
use std::collections::HashMap;

use crate::error::{Error, Result};
use crate::industry::utils::{fetch_ignored_jobs, resolve_container_names};
use crate::task::Task;
use crate::utils::additional_data;
use super::job_detection::job_detection;
use super::utils::{cleanup_delivered_jobs, fetch_done_job_ids, fetch_startable_jobs, insert_job_detection_log, resolve_main_character_from_corporation, update_finished_jobs, update_industry_jobs};

#[derive(Debug, Deserialize)]
struct AdditionalData {
    corporation_id: CorporationId,
}

pub async fn task(
    task:        &mut Task,
    pool:        &PgPool,
    credentials: &Credentials
) -> Result<()> {
    let additional_data = additional_data::<AdditionalData>(task)?;

    //if let Err(e) = cleanup_delivered_jobs(pool, *additional_data.corporation_id).await {
    //    tracing::warn!("{}", e);
    //    task.add_log(e.to_string());
    //}

    // create an eve api client
    let client = if let Some(client) = crate::utils::eve_api_client(
            credentials.clone(),
            (*additional_data.corporation_id).into(),
        ).await {

        client
    } else {
        return Err(Error::NoCredentials(*additional_data.corporation_id));
    };

    // fetch all industry jobs for the corporation
    let industry_jobs = match client
        .industry_corporation_jobs_copy()
        .await
        .map_err(|e| Error::ApiError(e)) {

        Ok(x) => x,
        Err(e) => {
            task.add_error(e.to_string());
            return Err(Error::NoOp);
        }
    };

    if industry_jobs.is_empty() {
        return Ok(());
    }

    let job_ids = industry_jobs
        .iter()
        .map(|x| *x.job_id)
        .collect::<Vec<_>>();

    let mut location_ids = industry_jobs
        .iter()
        .map(|x| x.location_id)
        .collect::<Vec<_>>();
    location_ids.sort();
    location_ids.dedup();

    let mut output_location_ids = industry_jobs
        .iter()
        .map(|x| x.output_location_id)
        .collect::<Vec<_>>();
    output_location_ids.sort();
    output_location_ids.dedup();

    let character_ids = resolve_main_character_from_corporation(
            pool,
            additional_data.corporation_id,
        )
        .await?;

    // resolve all container ids
    let container_names = match resolve_container_names(
        &pool,
        &client,
        &location_ids,
        &output_location_ids,
    ).await {
        Ok(x)  => x,
        Err(e) => {
            tracing::error!("Error resolving container names {}", e);
            task.add_error(e.to_string());
            HashMap::new()
        }
    };

    let startable_jobs = match fetch_startable_jobs(
        pool,
        character_ids,
    ).await {

        Ok(x)  => x,
        Err(e) => {
            task.add_error(e.to_string());
            Vec::new()
        }
    };

    let finished_job_ids = match fetch_done_job_ids(pool, job_ids).await {
        Ok(x)  => x,
        Err(e) => {
            task.add_error(e.to_string());
            Vec::new()
        }
    };

    let ignored_job_ids = match fetch_ignored_jobs(pool).await {
        Ok(x)  => x,
        Err(e) => {
            task.add_error(e.to_string());
            Vec::new()
        }
    };

    let mut updates      = HashMap::new();
    let mut unmatched    = Vec::new();
    let mut used_ids     = Vec::new();
    let mut used_job_ids = Vec::new();

    let mut blueprint_id          = Vec::new();
    let mut blueprint_location_id = Vec::new();
    let mut blueprint_type_id     = Vec::new();
    let mut facility_id           = Vec::new();
    let mut installer_id          = Vec::new();
    let mut job_id                = Vec::new();
    let mut runs                  = Vec::new();
    let mut cost                  = Vec::new();
    let mut end_date              = Vec::new();
    let mut activity              = Vec::new();

    for entry in industry_jobs.iter() {
        blueprint_id.push(*entry.blueprint_id);
        blueprint_location_id.push(*entry.blueprint_location_id);
        blueprint_type_id.push(*entry.blueprint_type_id);
        facility_id.push(entry.facility_id as i64);
        installer_id.push(*entry.installer_id);
        job_id.push(*entry.job_id as i32);
        runs.push(entry.runs as i32);
        cost.push(entry.cost.unwrap_or_default() as f32);
        end_date.push(entry.end_date.clone());
        activity.push(entry.activity.clone());

        let industry_jobs = industry_jobs
            .iter()
            .filter(|x|
                x.activity == IndustryActivity::Manufacturing ||
                x.activity == IndustryActivity::Reactions
            )
            .cloned()
            .collect::<Vec<_>>();
        let (updates_, unmatched_) = job_detection(
            &industry_jobs,
            &startable_jobs,
            &finished_job_ids,
            &ignored_job_ids,
            &container_names,
            &mut used_ids,
            &mut used_job_ids,
        );
        updates.extend(updates_);
        unmatched.extend(unmatched_);
    }

    // TODO: check if this is okay here
    //if let Err(e) = cleanup_delivered_jobs(pool, *additional_data.corporation_id).await {
    if let Err(e) = cleanup_delivered_jobs(pool).await {
        tracing::warn!("{}", e);
        task.add_log(e.to_string());
    }

    sqlx::query!("
            INSERT INTO industry_job
            (
                character_corporation_id,
                blueprint_id,
                blueprint_location_id,
                blueprint_type_id,
                facility_id,
                installer_id,
                job_id,
                runs,
                cost,
                end_date,
                activity
            )
            SELECT $1, * FROM UNNEST(
                $2::BIGINT[],
                $3::BIGINT[],
                $4::INTEGER[],
                $5::BIGINT[],
                $6::INTEGER[],
                $7::INTEGER[],
                $8::INTEGER[],
                $9::REAL[],
                $10::VARCHAR[],
                $11::INDUSTRY_ACTIVITY[]
            )
            ON CONFLICT (job_id)
            DO UPDATE SET is_delivered = false
        ",
            *additional_data.corporation_id,
            &blueprint_id,
            &blueprint_location_id,
            &blueprint_type_id,
            &facility_id,
            &installer_id,
            &job_id,
            &runs,
            &cost,
            &end_date,
            activity as _,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(Error::InsertIndustryJob)?;

    update_industry_jobs(pool, &updates).await?;
    update_finished_jobs(pool).await?;
    insert_job_detection_log(pool, &updates, &unmatched).await?;

    Ok(())
}
