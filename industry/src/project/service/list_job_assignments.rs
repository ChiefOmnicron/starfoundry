use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use starfoundry_lib_eve_gateway::{EveGatewayApiClient, Item};
use std::collections::HashMap;
use utoipa::ToSchema;

use crate::project::error::{ProjectError, Result};
use crate::project::ProjectAssignmentUuid;
use crate::project::service::ProjectJobUuid;
use crate::sort_by_job_flat;

pub async fn list_job_assignments(
    pool:                   &PgPool,
    assignment_id:          ProjectAssignmentUuid,
    eve_gateway_api_client: &impl EveGatewayApiClient,
) -> Result<Vec<ProjectJobAssignmentGroup>> {
    // TODO: merge end_date into project_job?
    let entries = sqlx::query!(r#"
            SELECT
                pja.id AS "id!",
                pja.job_id AS "job_id!",
                pja.project_id AS "project_id!",
                pja.started AS "started!",
                pj.runs,
                pj.structure_id,
                pj.type_id
            FROM project_job_assignment pja
            JOIN project_job pj ON pja.job_id = pj.id
            WHERE pja.id = $1
        "#,
            *assignment_id,
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

    let mut project_ids = entries
        .iter()
        .map(|x| x.project_id)
        .map(Into::into)
        .collect::<Vec<_>>();
    project_ids.sort();
    project_ids.dedup();

    let items = eve_gateway_api_client
        .fetch_item_bulk(type_ids)
        .await?
        .into_iter()
        .map(|x| (x.type_id, x))
        .collect::<HashMap<_, _>>();

    let structures = sqlx::query!("
            SELECT
                id,
                name
            FROM structure
            WHERE id = ANY($1)
        ",
            &structure_ids,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListJobs)?
        .into_iter()
        .map(|x| (x.id, x.name))
        .collect::<HashMap<_, _>>();
    let projects = sqlx::query!("
            SELECT
                id,
                name
            FROM project
            WHERE id = ANY($1)
        ",
            &project_ids,
        )
        .fetch_all(pool)
        .await
        .map_err(ProjectError::ListJobs)?
        .into_iter()
        .map(|x| (x.id, x.name))
        .collect::<HashMap<_, _>>();

    let mut project_jobs = HashMap::new();
    for entry in entries {
        let item = if let Some(x) = items.get(&entry.type_id.into()) {
            x.clone()
        } else {
            continue;
        };
        let structure = if let Some(x) = structures.get(&entry.structure_id.into()) {
            x
        } else {
            continue;
        };
        let project = if let Some(x) = projects.get(&entry.project_id.into()) {
            x.clone()
        } else {
            continue;
        };

        let job = ProjectJobAssignment {
            id:             entry.job_id.into(),
            structure_name: structure.clone(),
            started:        entry.started,
            item:           item,
            runs:           entry.runs,
        };
        project_jobs
            .entry(project)
            .and_modify(|x: &mut Vec<ProjectJobAssignment>| x.push(job.clone()))
            .or_insert(vec![job]);
    }

    let mut jobs = Vec::new();
    project_jobs
        .into_iter()
        .for_each(|(header, entries)| {
            let entries = sort_jobs(entries);
            jobs.push(ProjectJobAssignmentGroup {
                header,
                entries,
            });
        });
    jobs.sort_by_key(|x| x.header.clone());

    Ok(jobs)
}

sort_by_job_flat!(sort_jobs, ProjectJobAssignment);

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJobAssignment {
    pub id:             ProjectJobUuid,
    pub structure_name: String,
    pub started:        bool,
    pub item:           Item,
    pub runs:           i32,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct ProjectJobAssignmentGroup {
    pub header:  String,
    pub entries: Vec<ProjectJobAssignment>,
}
