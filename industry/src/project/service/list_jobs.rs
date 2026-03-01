use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_industry::Structure;
use starfoundry_lib_types::{CharacterId, TypeId};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use crate::structure::service::FetchStructureQuery;

pub async fn list_jobs(
    pool:                   &PgPool,
    character_id:           CharacterId,
    project_id:             ProjectUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Vec<ProjectJobGroup>> {
    let entries = sqlx::query!(r#"
            SELECT
                id,
                runs,
                status AS "status: ProjectJobStatusDatabase",
                cost,
                job_id,
                structure_id,
                type_id,
                started_by
            FROM project_job
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

        let project_group = ProjectJob {
            id:         entry.id.into(),
            job_id:     entry.job_id,
            status:     entry.status.into(),

            cost:       entry.cost,
            runs:       entry.runs,

            item:       item.clone(),
            structure:  structure.clone(),
            started_by: entry.started_by.map(Into::into),
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

    Ok(sort_jobs(project_jobs))
}

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

    let mut done = entries
        .iter()
        .filter(|x| x.status == ProjectJobStatus::Done)
        .map(|x| x.item.type_id)
        .map(Into::into)
        .collect::<Vec<TypeId>>();
    // bought market data is considered as done, and qualifies for something
    // to be ready to start
    done.extend(market_data);

    let building = entries
        .iter()
        .filter(|x| x.status == ProjectJobStatus::Building)
        .map(|x| x.item.type_id)
        .map(Into::into)
        .collect::<Vec<TypeId>>();

    for entry in entries.iter_mut() {
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

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJob {
    pub id:         Uuid,
    pub job_id:     Option<i32>,
    pub status:     ProjectJobStatus,

    pub runs:       i32,
    pub cost:       Option<f64>,

    pub item:       Item,
    pub structure:  Structure,
    pub started_by: Option<CharacterId>,
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

fn sort_jobs(
    entries: Vec<ProjectJob>,
) -> Vec<ProjectJobGroup> {
    let mut job_lists       = Vec::new();
    let mut grouped_entries = std::collections::HashMap::new();

    let mut insert_into_map = |id: i32, entry: ProjectJob| {
        grouped_entries
            .entry(id)
            .and_modify(|x: &mut Vec<ProjectJob>| x.push(entry.clone()))
            .or_insert(vec![entry]);
    };

    // First go through all entries, and sort them into the map
    for entry in entries.into_iter() {
        match *entry.item.category.category_id {
            6i32 => {
                insert_into_map(6, entry);
                continue;
            },
            8i32 => {
                insert_into_map(8, entry);
                continue;
            },
            _  => {}
        }

        match *entry.item.group.group_id {
            332i32 => {
                insert_into_map(332, entry);
                continue;
            },
            334i32 => {
                insert_into_map(334, entry);
                continue;
            },
            428i32 => {
                insert_into_map(428, entry);
                continue;
            },
            429i32 => {
                insert_into_map(429, entry);
                continue;
            },
            873i32 => {
                insert_into_map(873, entry);
                continue;
            },
            913i32 => {
                insert_into_map(913, entry);
                continue;
            },
            974i32 => {
                insert_into_map(974, entry);
                continue;
            },
            4096i32 => {
                insert_into_map(4096, entry);
                continue;
            },
            _  => {}
        }

        if let Some(ref meta_group) = entry.item.meta_group {
            match **meta_group {
                1i32 => {
                    insert_into_map(1, entry);
                    continue;
                },
                2i32 => {
                    insert_into_map(2, entry);
                    continue;
                },
                8i32 => {
                    insert_into_map(8, entry);
                    continue;
                },
                _  => {}
            }
        }

        insert_into_map(0, entry.clone());
    }

    for (header, id) in vec![
        ("INTERMEDIATE_REACTIONS",                    428),
        ("COMPOSITE_REACTIONS",                       429),
        ("BIOCHEM_REACTIONS",                        4096),
        ("HYBRID_REACTIONS",                          974),
        ("CONSTRUCTION_COMPONENTS",                   334),
        ("ADVANCED_CAPITAL_CONSTRUCTION_COMPONENTS",  913),
        ("CAPITAL_CONSTRUCTION_COMPONENTS",           873),
        ("TOOLS",                                     332),
        ("T1_STUFF",                                    1),
        ("T2_STUFF",                                    2),
        ("CHARGES",                                     8),
        ("SHIPS",                                       6),
        ("UNKNOWN",                                     0),
    ] {
        if let Some(entries) = grouped_entries.get_mut(&id) {
            entries.sort_by_key(|x| x.item.name.clone());
            let mut entries = entries
                .chunk_by(|a, b| a.item.name == b.item.name)
                .map(|x| x.into())
                .collect::<Vec<Vec<ProjectJob>>>();
            let entries = entries
                .iter_mut()
                .map(|x| {
                    x.sort_by_key(|y| y.runs);
                    x.reverse();
                    x.clone()
                })
                .collect::<Vec<_>>()
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            job_lists.push(
                ProjectJobGroup {
                    header:  header.into(),
                    entries: entries.clone(),
                }
            );
        }
    }

    job_lists
}
