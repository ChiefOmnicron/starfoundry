use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_industry::Structure;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use crate::structure::service::FetchStructureQuery;
use crate::sort_by_job;
use crate::project::service::ProjectJobUuid;

pub async fn list_jobs(
    pool:                   &PgPool,
    character_id:           CharacterId,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
    filter:                 ProjectJobFilter,
) -> Result<Vec<ProjectJobGroup>> {
    // TODO: merge end_date into project_job?
    let entries = sqlx::query!(r#"
            SELECT
                pj.id,
                pj.runs,
                pj.status AS "status: ProjectJobStatusDatabase",
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
            status:     entry.status.into(),

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

async fn determine_ready_to_start(
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
    done.extend(vec![62402, 62404, 62406, 62399, 62405, 62397, 62398, 62403, 62400].into_iter().map(Into::<TypeId>::into).collect::<Vec<_>>());
    done.extend(vec![62377, 62379, 62380, 62381, 62382, 62383, 62384, 62385].into_iter().map(Into::<TypeId>::into).collect::<Vec<_>>());
    done.extend(vec![62396, 62386, 62387, 62390, 62391, 62392, 62393, 62394].into_iter().map(Into::<TypeId>::into).collect::<Vec<_>>());

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
pub struct ProjectJob {
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
pub struct ProjectJobGroup {
    pub header:  String,
    pub entries: Vec<ProjectJob>,
}

#[derive(
    Clone, Debug, Copy,
    PartialEq, Eq, PartialOrd, Ord,
    Deserialize, Serialize, ToSchema
)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectJobStatus {
    WaitingForMaterials,
    ReadyToStart,
    Building,
    Done,
}

impl From<ProjectJobStatusDatabase> for ProjectJobStatus {
    fn from(value: ProjectJobStatusDatabase) -> Self {
        match value {
            ProjectJobStatusDatabase::WaitingForMaterials => Self::WaitingForMaterials,
            ProjectJobStatusDatabase::Building => Self::Building,
            ProjectJobStatusDatabase::Done => Self::Done,
        }
    }
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

#[derive(Debug, Default, Deserialize, ToSchema, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct ProjectJobFilter {
    #[serde(default)]
    pub startable: Option<bool>,
}
