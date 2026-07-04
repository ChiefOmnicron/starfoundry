use chrono::NaiveDateTime;
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient};
use starfoundry_lib_industry::project::{ProjectJob, ProjectJobFilter, ProjectJobGroup, ProjectJobStatus};
use starfoundry_lib_industry::ProjectUuid;
use starfoundry_lib_market::Gas;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::collections::HashMap;

use crate::project::error::{ProjectError, Result};
use crate::sort_by_job;
use crate::structure::service::FetchStructureQuery;

pub async fn list_jobs(
    pool:                   &PgPool,
    character_id:           CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    project_id:             ProjectUuid,
    filter:                 ProjectJobFilter,
) -> Result<Vec<ProjectJobGroup>> {
    // TODO: merge end_date into project_job?
    let entries = sqlx::query!(r#"
            SELECT
                pj.id,
                pj.runs,
                pj.status AS "status: ProjectJobStatus",
                pj.cost,
                pj.job_id,
                pj.structure_id,
                pj.type_id,
                pj.started_by,
                ij.end_date AS "end_date?"
            FROM project_job pj
            LEFT JOIN industry_job ij ON pj.job_id = ij.job_id
            WHERE project_id = $1
        "#,
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListJobs)?;

    let mut type_ids = entries
        .iter()
        .map(|x| x.type_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    type_ids.sort();
    type_ids.dedup();

    let mut structure_ids = entries
        .iter()
        .map(|x| x.structure_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    structure_ids.sort();
    structure_ids.dedup();

    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();
    let structures = crate::structure::service::fetch_bulk(
            pool,
            eve_gateway_api_client,
            character_id,
            structure_ids,
            FetchStructureQuery::default(),
        )
        .await?
        .into_iter()
        .map(|x| (x.id, x))
        .collect::<HashMap<_, _>>();

    let mut project_jobs = Vec::new();
    for entry in entries {
        let item = if let Some(x) = items.get(&entry.type_id.into()) {
            x
        } else {
            continue;
        };
        let structure = if let Some(x) = structures.get(&entry.structure_id.into()) {
            x
        } else {
            continue;
        };

        let end_date = if let Some(x) = entry.end_date {
            NaiveDateTime::parse_from_str(&x, "%Y-%m-%dT%H:%M:%SZ")
                .map(|x| Some(x))
                .unwrap_or(None)
        } else {
            None
        };

        let project_group = ProjectJob {
            id:         entry.id.into(),
            project_id: project_id,
            job_id:     entry.job_id,
            status:     entry.status,

            cost:       entry.cost,
            runs:       entry.runs,

            item:       item.clone(),
            structure:  structure.clone(),
            started_by: entry.started_by.map(Into::into),

            end_date:   end_date,
        };
        project_jobs.push(project_group);
    }

    determine_ready_to_start(
            pool,
            project_id,
            eve_gateway_api_client,
            &mut project_jobs,
        )
        .await?;

    let project_jobs = if let Some(_) = filter.startable {
        project_jobs
            .into_iter()
            .filter(|y| y.status == ProjectJobStatus::ReadyToStart)
            .collect::<Vec<_>>()
    } else {
        project_jobs
    };

    Ok(sort_jobs(project_jobs))
}

sort_by_job!(sort_jobs, ProjectJob, ProjectJobGroup);

pub async fn determine_ready_to_start(
    pool:                   &PgPool,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    entries:                &mut Vec<ProjectJob>,
) -> Result<()> {
    let mut waiting_for_materials = entries
        .iter()
        .filter(|x| x.status == ProjectJobStatus::WaitingForMaterials)
        .map(|x| x.item.type_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    waiting_for_materials.sort();
    waiting_for_materials.dedup();

    let dependencies = eve_gateway_api_client
        .fetch_blueprint_dependencies_bulk(waiting_for_materials)
        .await?
        .into_iter()
        .map(|x| (x.product_type_id, x))
        .collect::<HashMap<_, _>>();

    let market_data = bought_materials(
            pool,
            project_id,
        )
        .await?;
    let stock_data = used_stock(
            pool,
            project_id,
        )
        .await?;

    let mut done = entries
        .iter()
        .filter(|x| x.status == ProjectJobStatus::Done)
        .map(|x| x.item.type_id)
        .map(Into::into)
        .collect::<Vec<TypeId>>();
    // bought market data is considered as done, and qualifies for something
    // to be ready to start
    done.extend(market_data);
    done.extend(stock_data);
    // TODO: remove when market is implemented
    done.extend(vec![34, 35, 36, 37, 38, 39, 40].into_iter().map(Into::<TypeId>::into).collect::<Vec<_>>());
    // R.A.M
    done.extend(vec![11476, 11475, 11485, 11483, 11482, 11481, 11484, 11478, 11486].into_iter().map(Into::<TypeId>::into).collect::<Vec<_>>());
    done.extend(Gas::compressed_type_ids());
    done.extend(Gas::uncompressed_type_ids());

    let building = entries
        .iter()
        .filter(|x| x.status == ProjectJobStatus::Building)
        .map(|x| x.item.type_id)
        .map(Into::into)
        .collect::<Vec<TypeId>>();

    for entry in entries.iter_mut() {
        if entry.status != ProjectJobStatus::WaitingForMaterials {
            continue;
        }

        let dependency = if let Some(dependency) = dependencies.get(&entry.item.type_id) {
            dependency
        } else {
            continue;
        };

        // if a dependency is still building ignore it
        // but if some of them are already done, keep it
        let has_dependency_building = dependency
            .depends_on
            .iter()
            .any(|x| building.contains(x) && !done.contains(x));
        if has_dependency_building {
            continue;
        }

        // if all dependencies are done, define it as ready to start
        let all_done = dependency
            .depends_on
            .iter()
            .all(|x| done.contains(x));
        if all_done {
            entry.status = ProjectJobStatus::ReadyToStart;
        }
    }

    Ok(())
}

async fn bought_materials(
    pool:      &PgPool,
    project_id: ProjectUuid,
) -> Result<Vec<TypeId>> {
    sqlx::query!("
            SELECT type_id
            FROM project_market
            WHERE project_id = $1
            AND cost IS NOT NULL
        ",
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map(|x| {
            x
                .into_iter()
                .map(|y| y.type_id)
                .map(Into::into)
                .collect::<Vec<_>>()
        })
        .map_err(ProjectError::ListJobs)
}

async fn used_stock(
    pool:      &PgPool,
    project_id: ProjectUuid,
) -> Result<Vec<TypeId>> {
    sqlx::query!("
            SELECT type_id
            FROM project_stock
            WHERE project_id = $1
        ",
            *project_id,
        )
        .fetch_all(pool)
        .await
        .map(|x| {
            x
                .into_iter()
                .map(|y| y.type_id)
                .map(Into::into)
                .collect::<Vec<_>>()
        })
        .map_err(ProjectError::ListJobs)
}
