use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_industry::Structure;
use starfoundry_lib_market::Gas;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::project::error::{ProjectError, Result};
use crate::project::{ProjectJobUuid, ProjectUuid};
use crate::structure::service::FetchStructureQuery;
use crate::{sort_by_job_flat};
use crate::project::service::{ProjectFilter, ProjectJobStatus, ProjectJobStatusDatabase, list};

pub async fn list_all_jobs(
    pool:                   &PgPool,
    character_id:           CharacterId,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Vec<ProjectJobAllGroup>> {
    let projects = list(
            pool,
            character_id,
            ProjectFilter {
                status: Some("IN_PROGRESS".into()),
                ..Default::default()
            },
        )
        .await?;
    let project_ids = projects
        .iter()
        .map(|x| *x.id)
        .collect::<Vec<_>>();

    // TODO: merge end_date into project_job?
    // TODO: find out why in github test all the force overwrites are required
    let entries = sqlx::query!(r#"
            SELECT
                pj.id           AS "id!",
                pj.runs         AS "runs!",
                pj.status       AS "status!: ProjectJobStatusDatabase",
                pj.cost,
                pj.job_id,
                pj.structure_id AS "structure_id!",
                pj.type_id      AS "type_id!",
                pj.started_by,
                pj.project_id   AS "project_id!",
                ij.end_date     AS "end_date?"
            FROM project_job pj
            LEFT JOIN industry_job ij ON pj.job_id = ij.job_id
            WHERE project_id = ANY($1)
        "#,
            &project_ids,
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

    let mut project_jobs = HashMap::new();
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

        let project_job = ProjectJobAll {
            id:         entry.id.into(),
            project_id: entry.project_id.into(),
            job_id:     entry.job_id,
            status:     entry.status.into(),

            cost:       entry.cost,
            runs:       entry.runs,

            item:       item.clone(),
            structure:  structure.clone(),
            started_by: entry.started_by.map(Into::into),

            end_date:   end_date,
        };

        project_jobs
            .entry(entry.project_id)
            .and_modify(|x: &mut Vec<ProjectJobAll>| x.push(project_job.clone()))
            .or_insert(vec![project_job]);
    }

    for (project_id, mut jobs) in project_jobs.iter_mut() {
        determine_ready_to_start(
                pool,
                (*project_id).into(),
                eve_gateway_api_client,
                &mut jobs,
            )
            .await?;
    }

    let mut result = Vec::new();
    for (project_id, jobs) in project_jobs.iter() {
        let project_name = if let Some(x) = projects
            .iter()
            .find(|x| x.id == (*project_id).into())
            .map(|x| x.name.clone()) {

            x
        } else {
            continue;
        };

        let jobs = jobs
                    .into_iter()
                    .filter(|x| x.status == ProjectJobStatus::ReadyToStart)
                    .cloned()
                    .collect::<Vec<_>>();
        if jobs.is_empty() {
            continue;
        }

        let group = ProjectJobAllGroup {
            header:     project_name,
            project_id: (*project_id).into(),
            entries:    sort_jobs(jobs),
        };
        result.push(group);
    }
    result.sort_by_key(|x| x.header.clone());

    Ok(result)
}

sort_by_job_flat!(sort_jobs, ProjectJobAll);

async fn determine_ready_to_start(
    pool:                   &PgPool,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    entries:                &mut Vec<ProjectJobAll>,
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
    done.extend(Gas::compressed_type_ids());
    done.extend(Gas::uncompressed_type_ids());

    let building = entries
        .iter()
        .filter(|x| x.status == ProjectJobStatus::Building)
        .map(|x| x.item.type_id)
        .map(Into::into)
        .collect::<Vec<TypeId>>();

    for entry in entries.iter_mut() {
        if entry.status == ProjectJobStatus::Done {
            continue;
        }

        let dependency = if let Some(dependency) = dependencies.get(&entry.item.type_id) {
            dependency
        } else {
            continue;
        };

        // if a dependency is still building, ignore it
        let has_dependency_building = dependency
            .depends_on
            .iter()
            .any(|x| building.contains(x));
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

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJobAll {
    pub id:         ProjectJobUuid,
    pub project_id: ProjectUuid,
    pub job_id:     Option<i32>,
    pub status:     ProjectJobStatus,

    pub runs:       i32,
    pub cost:       Option<f64>,

    pub item:       Item,
    pub structure:  Structure,
    pub started_by: Option<CharacterId>,

    pub end_date:   Option<NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJobAllGroup {
    pub header:     String,
    pub project_id: ProjectUuid,
    pub entries:    Vec<ProjectJobAll>,
}
