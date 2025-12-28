use sqlx::PgPool;
use std::collections::HashMap;
use uuid::{NoContext, Timestamp, Uuid};

use crate::Mapping;

pub async fn migrate_project(
    postgres_source:       &PgPool,
    postgres_destination:  &PgPool,
    project_group_mapping: &Mapping,
    structure_mappings:    &Mapping,
) -> Result<Mapping, Box<dyn std::error::Error>> {
    dbg!("Start - project");
    let mut mappings = HashMap::new();

    // TODO: the field structure_group_id is no more, needs to be properly migrated

    let projects = sqlx::query!(r#"
            SELECT
                id,
                name,
                owner,
                status AS "status: ProjectStatus",
                sell_price,
                orderer,
                note,
                --structure_group_id,
                project_group_id,
                created_at,
                updated_at
            FROM project
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for project in projects {
        let timestamp = Timestamp::from_unix(NoContext, project.created_at.timestamp() as u64, 0);
        let project_id = Uuid::new_v7(timestamp);
        mappings.insert(project.id, project_id);

        if let None = project_group_mapping.get(&project.project_group_id) {
            mappings.remove(&project.id);
            continue;
        }

        let project_group_id = project_group_mapping.get(&project.project_group_id).unwrap();

        sqlx::query!("
                INSERT INTO project (
                    id,
                    name,
                    owner,
                    status,
                    sell_price,
                    orderer,
                    note,
                    --structure_group_id,
                    project_group_id,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ",
                project_id,
                project.name,
                project.owner,
                project.status as _,
                project.sell_price,
                project.orderer,
                project.note,
                //structure_group_id,
                project_group_id,
                project.created_at,
                project.updated_at
            )
            .execute(&mut *transaction)
            .await?;
    }

    let jobs = sqlx::query!(r#"
            SELECT
                project_id,
                type_id,
                runs,
                status AS "status: ProjectJobStatus",
                cost,
                id,
                job_id,
                structure_id,
                character_id,
                created_at,
                updated_at
            FROM project_job
        "#)
        .fetch_all(postgres_source)
        .await?;
    for job in jobs {
        let timestamp = Timestamp::from_unix(NoContext, job.created_at.timestamp() as u64, 0);
        let job_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&job.project_id) {
            x
        } else {
            continue;
        };
        let structure_id =  if let Some(x) = structure_mappings.get(&job.structure_id) {
            x
        } else {
            &Uuid::default()
        };

        sqlx::query!("
                INSERT INTO project_job (
                    project_id,
                    type_id,
                    runs,
                    status,
                    cost,
                    id,
                    job_id,
                    structure_id,
                    started_by,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            ",
                project_id,
                job.type_id,
                job.runs,
                job.status as _,
                job.cost,
                job_id,
                job.job_id,
                structure_id,
                job.character_id,
                job.created_at,
                job.updated_at
            )
            .execute(&mut *transaction)
            .await?;
    }

    transaction.commit().await?;
    dbg!("Done - project");

    Ok(mappings)
}

#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type
)]
#[sqlx(type_name = "PROJECT_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectStatus {
    /// the project has not started yet, but materials are gathered
    /// job detection not active
    Preparing,
    /// the project is currently in progress, and job detection is active
    InProgress,
    /// the project is currently paused, job detection not active
    Paused,
    /// the project is finished, industry job detection is no longer active
    Done,
}

#[derive(
    Clone, Debug, Copy, Hash,
    PartialEq, Eq, PartialOrd, Ord,
    sqlx::Type,
)]
#[sqlx(type_name = "PROJECT_JOB_STATUS")]
#[sqlx(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ProjectJobStatus {
    WaitingForMaterials,
    Building,
    Done,
}
