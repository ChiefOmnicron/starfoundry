use sqlx::PgPool;
use uuid::{NoContext, Timestamp, Uuid};

use crate::Mapping;

pub async fn migrate_project(
    postgres_source:       &PgPool,
    postgres_destination:  &PgPool,
    mappings:              &mut Mapping,
) -> Result<(), Box<dyn std::error::Error>> {
    tracing::info!("Start - project");

    let projects = sqlx::query!(r#"
            SELECT
                id,
                name,
                owner,
                status AS "status: ProjectStatus",
                sell_price,
                orderer,
                note,
                structure_group_id,
                project_group_id,
                created_at,
                updated_at
            FROM project
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for project in projects {
        let project_id = if let Some(x) = mappings.get(&project.id) {
            x.clone()
        } else {
            let timestamp = Timestamp::from_unix(NoContext, project.created_at.timestamp() as u64, 0);
            let project_id = Uuid::new_v7(timestamp);
            mappings.insert(project.id, project_id);
            project_id
        };

        if let None = mappings.get(&project.project_group_id) {
            mappings.remove(&project.id);
            continue;
        }

        let project_group_id = mappings.get(&project.project_group_id).unwrap().clone();
        let industry_hub_id = if let Some(x) = mappings.get(&project.structure_group_id) {
            x.clone()
        } else {
            Uuid::default()
        };

        let solution_id = sqlx::query!("
                INSERT INTO solution (
                    industry_hub_id,
                    project_group_id
                )
                VALUES($1, $2)
                RETURNING id
            ",
                &industry_hub_id,
                project_group_id,
            )
            .fetch_one(postgres_destination)
            .await?
            .id;
        mappings.insert(project_id, solution_id);

        let status: String = match project.status {
            ProjectStatus::Done         => "DONE",
            ProjectStatus::InProgress   => "IN_PROGRESS",
            ProjectStatus::Paused       => "PAUSED",
            ProjectStatus::Preparing    => "READY_TO_START",
        }.into();

        sqlx::query!("
                INSERT INTO project (
                    id,
                    name,
                    owner,
                    status,
                    sell_price,
                    orderer,
                    note,
                    solution_id,
                    project_group_id,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4::PROJECT_STATUS, $5, $6, $7, $8, $9, $10, $11)
                ON CONFLICT (id)
                DO UPDATE SET
                    name        = EXCLUDED.name,
                    status      = EXCLUDED.status,
                    sell_price  = EXCLUDED.sell_price,
                    orderer     = EXCLUDED.orderer,
                    note        = EXCLUDED.note,
                    updated_at  = EXCLUDED.updated_at
            ",
                project_id,
                project.name,
                project.owner,
                status as _,
                project.sell_price,
                project.orderer,
                project.note,
                solution_id,
                project_group_id,
                project.created_at,
                project.updated_at
            )
            //.execute(&mut *transaction)
            .execute(postgres_destination)
            .await?;
    }
    tracing::info!("[project] projects migrated");

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
    sqlx::query!("
            DELETE FROM project_job
        ")
        .execute(&mut *transaction)
        .await?;
    for job in jobs {
        let timestamp = Timestamp::from_unix(NoContext, job.created_at.timestamp() as u64, 0);
        let job_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&job.project_id) {
            x
        } else {
            continue;
        };
        let structure_id =  if let Some(x) = mappings.get(&job.structure_id) {
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
    tracing::info!("[project] project jobs migrated");

    let misc_entries = sqlx::query!(r#"
            SELECT
                project_id,
                item,
                cost,
                quantity,
                description,
                created_at,
                updated_at
            FROM project_misc
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_misc
        ")
        .execute(&mut *transaction)
        .await?;
    for misc in misc_entries {
        let timestamp = Timestamp::from_unix(NoContext, misc.created_at.timestamp() as u64, 0);
        let misc_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&misc.project_id) {
            x
        } else {
            continue;
        };

        sqlx::query!("
                INSERT INTO project_misc (
                    project_id,
                    id,
                    item,
                    cost,
                    quantity,
                    description,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ",
                project_id,
                misc_id,
                misc.item,
                misc.cost,
                misc.quantity,
                misc.description,
                misc.created_at,
                misc.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    tracing::info!("[project] project misc migrated");

    let market_entries = sqlx::query!(r#"
            SELECT
                project_id,
                type_id,
                quantity,
                cost,
                source,
                created_at,
                updated_at
            FROM project_market
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_market
        ")
        .execute(&mut *transaction)
        .await?;
    sqlx::query!("
            DELETE FROM solution_material
        ")
        .execute(&mut *transaction)
        .await?;
    for market in market_entries {
        let timestamp = Timestamp::from_unix(NoContext, market.created_at.timestamp() as u64, 0);
        let market_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&market.project_id.unwrap()) {
            x
        } else {
            continue;
        };

        sqlx::query!("
                INSERT INTO project_market (
                    project_id,
                    id,
                    type_id,
                    quantity,
                    cost,
                    source,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ",
                project_id,
                market_id,
                market.type_id,
                market.quantity,
                market.cost,
                market.source,
                market.created_at,
                market.updated_at,
            )
            .execute(&mut *transaction)
            .await?;

        let solution_id = mappings.get(&project_id).unwrap();
        sqlx::query!("
                INSERT INTO solution_material (
                    solution_id,
                    id,
                    type_id,
                    quantity,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
                solution_id,
                market_id,
                market.type_id,
                market.quantity,
                market.created_at,
                market.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    tracing::info!("[project] project market migrated");

    let stock_entries = sqlx::query!(r#"
            SELECT
                project_id,
                type_id,
                quantity,
                cost,
                created_at,
                updated_at
            FROM project_stock
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_stock
        ")
        .execute(&mut *transaction)
        .await?;
    sqlx::query!("
            DELETE FROM solution_stock
        ")
        .execute(&mut *transaction)
        .await?;
    for stock in stock_entries {
        let timestamp = Timestamp::from_unix(NoContext, stock.created_at.timestamp() as u64, 0);
        let stock_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&stock.project_id) {
            x
        } else {
            continue;
        };

        sqlx::query!("
                INSERT INTO project_stock (
                    project_id,
                    id,
                    type_id,
                    quantity,
                    cost,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            ",
                project_id,
                stock_id,
                stock.type_id,
                stock.quantity,
                stock.cost,
                stock.created_at,
                stock.updated_at,
            )
            .execute(&mut *transaction)
            .await?;

        let solution_id = mappings.get(&project_id).unwrap();
        sqlx::query!("
                INSERT INTO solution_stock (
                    solution_id,
                    id,
                    type_id,
                    quantity,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
                solution_id,
                stock_id,
                stock.type_id,
                stock.quantity,
                stock.created_at,
                stock.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    tracing::info!("[project] project stock migrated");

    let excess_entries = sqlx::query!(r#"
            SELECT
                project_id,
                type_id,
                quantity,
                cost,
                created_at,
                updated_at
            FROM project_excess
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_excess
        ")
        .execute(&mut *transaction)
        .await?;
    sqlx::query!("
            DELETE FROM solution_excess
        ")
        .execute(&mut *transaction)
        .await?;
    for excess in excess_entries {
        let project_id =  if let Some(x) = mappings.get(&excess.project_id) {
            x
        } else {
            continue;
        };

        sqlx::query!("
                INSERT INTO project_excess (
                    project_id,
                    type_id,
                    quantity,
                    cost,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
                project_id,
                excess.type_id,
                excess.quantity,
                excess.cost,
                excess.created_at,
                excess.updated_at,
            )
            .execute(&mut *transaction)
            .await?;

        let solution_id = mappings.get(&project_id).unwrap();
        sqlx::query!("
                INSERT INTO solution_excess (
                    solution_id,
                    type_id,
                    quantity,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5)
            ",
                solution_id,
                excess.type_id,
                excess.quantity,
                excess.created_at,
                excess.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    tracing::info!("[project] project excess migrated");

    let product_entries = sqlx::query!(r#"
            SELECT
                id,
                project_id,
                type_id,
                quantity,
                material_efficiency,
                created_at,
                updated_at
            FROM project_product
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM solution_product
        ")
        .execute(&mut *transaction)
        .await?;
    for product in product_entries {
        let project_id = if let Some(x) = mappings.get(&product.project_id) {
            x
        } else {
            continue;
        };
        let solution_id = mappings.get(&project_id).unwrap();

        sqlx::query!("
                INSERT INTO solution_product (
                    id,
                    solution_id,
                    type_id,
                    quantity,
                    material_efficiency,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            ",
                product.id,
                solution_id,
                product.type_id,
                product.quantity,
                product.material_efficiency,
                product.created_at,
                product.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }

    tracing::info!("[project] project blacklist migrated");
    let product_entries = sqlx::query!(r#"
            SELECT
                project_id,
                type_id,
                created_at,
                updated_at
            FROM project_blacklist
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM solution_blacklist
        ")
        .execute(&mut *transaction)
        .await?;
    for product in product_entries {
        let project_id = if let Some(x) = mappings.get(&product.project_id) {
            x
        } else {
            continue;
        };
        let solution_id = mappings.get(&project_id).unwrap();

        sqlx::query!("
                INSERT INTO solution_blacklist (
                    solution_id,
                    type_id,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4)
            ",
                solution_id,
                product.type_id,
                product.created_at,
                product.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    tracing::info!("[project] project blacklist migrated");

    transaction.commit().await?;
    tracing::info!("Done - project");

    Ok(())
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
