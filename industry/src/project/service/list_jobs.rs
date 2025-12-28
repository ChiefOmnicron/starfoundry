use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectUuid;
use crate::structure::service::{FetchStructureQuery, Structure};

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
                status AS "status: ProjectJobStatus",
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
        .map_err(ProjectError::ListProjectJobs)?;

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
            status:     entry.status,

            cost:       entry.cost,
            runs:       entry.runs,

            item:       item.clone(),
            structure:  structure.clone(),
            started_by: entry.started_by.map(Into::into),
        };
        project_jobs.push(project_group);
    }

    Ok(sort_jobs(project_jobs))
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJob {
    pub id:         ProjectUuid,
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
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type, Deserialize, Serialize, ToSchema,
)]
#[sqlx(type_name = "PROJECT_JOB_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectJobStatus {
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

#[cfg(test)]
mod list_project_group_test {
    use sqlx::PgPool;
    use starfoundry_lib_types::CharacterId;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::test_util::EveGatewayTestApiClient;
    use crate::project::ProjectUuid;

    #[sqlx::test(
        fixtures(
            path = "../fixtures",
            scripts("base")
        ),
    )]
    async fn happy_path(
        pool: PgPool,
    ) {
        let gateway_client = EveGatewayTestApiClient::new();
        let result = super::list_jobs(
                &pool,
                CharacterId(1),
                ProjectUuid(Uuid::from_str("00000000-0000-0000-0000-000000000101").unwrap()),
                &gateway_client,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap()[0].entries.len(), 2);

        let result = super::list_jobs(
                &pool,
                CharacterId(1),
                ProjectUuid(Uuid::from_str("00000000-0000-0000-0000-000000000102").unwrap()),
                &gateway_client,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap()[0].entries.len(), 1);

        let result = super::list_jobs(
                &pool,
                CharacterId(1),
                ProjectUuid(Uuid::from_str("00000000-0000-0000-0000-000000000000").unwrap()),
                &gateway_client,
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }
}
