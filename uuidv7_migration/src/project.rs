use sqlx::PgPool;
use uuid::{NoContext, Timestamp, Uuid};

use crate::Mapping;
use std::str::FromStr;
use std::collections::HashSet;

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
                pj.project_id,
                pj.type_id,
                pj.runs,
                pj.status AS "status: ProjectJobStatus",
                pj.cost,
                pj.id,
                pj.job_id,
                pj.structure_id,
                pj.character_id,
                pj.created_at,
                pj.updated_at,
                p.name AS project_name,
                s.name AS structure_name
            FROM project_job pj
            JOIN project p ON p.id = pj.project_id
            JOIN structure s ON s.id = pj.structure_id
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_job
        ")
        .execute(&mut *transaction)
        .await?;

    let mut visited_names = HashSet::new();
    for (index, job) in jobs.iter().enumerate() {
        if visited_names.contains(&job.project_name) {
            continue;
        }

        tracing::info!("[{:6} / {:6}] Mapping - {}", index, jobs.len(), job.project_name);
        if job.project_name == "Pips Internal Order - Small Ancillary Current Routers" {
            visited_names.insert("Pips Internal Order - Small Ancillary Current Routers".to_string());
            continue
        }
        if job.project_name == "RCI Azbel fighters" {
            visited_names.insert("RCI Azbel fighters".to_string());
            continue
        }
        if job.project_name == "X2- Rokhs" {
            visited_names.insert("X2- Rokhs".to_string());
            continue
        }
        if job.project_name == "Fighters" {
            visited_names.insert("Fighters".to_string());
            continue
        }
        if job.project_name == "Fighters 2" {
            visited_names.insert("Fighters 2".to_string());
            continue
        }
        if job.project_name == "_aself" {
            visited_names.insert("_aself".to_string());
            continue
        }
        if job.project_name == "asdasd" {
            visited_names.insert("asdasd".to_string());
            continue
        }
        if job.project_name == "_test_rag" {
            visited_names.insert("_test_rag".to_string());
            continue
        }
        if job.project_name == "_test_hel" {
            visited_names.insert("_test_hel".to_string());
            continue
        }
        if job.project_name == "_test_rni" {
            visited_names.insert("_test_rni".to_string());
            continue
        }
        if job.project_name == "asd" {
            visited_names.insert("asd".to_string());
            continue
        }
        if job.project_name == "Alcatraz202 Enhanced Neurolink Protection Cell" {
            visited_names.insert("Alcatraz202 Enhanced Neurolink Protection Cell".to_string());
            continue
        }

        let project_id = &sqlx::query!("
                SELECT id
                FROM project
                WHERE name = $1
            ",
                job.project_name,
            )
            .fetch_one(postgres_destination)
            .await
            .unwrap()
            .id;

        let structure_id = if job.structure_name.starts_with("K7D-II") {
            &Uuid::default()
        } else if job.structure_name.starts_with("UVHO") {
            &Uuid::default()
        } else if job.structure_name.starts_with("A3-LOG") {
            &Uuid::default()
        } else if job.structure_name.starts_with("E3OI") {
            &Uuid::default()
        } else if job.structure_name.starts_with("A4B") {
            &Uuid::default()
        } else if job.structure_name.starts_with("RCI") {
            &Uuid::default()
        } else if job.structure_name.starts_with("31X") {
            &Uuid::default()
        } else if job.structure_name.starts_with("IGE") {
            &Uuid::default()
        } else if job.structure_name.starts_with("39P") {
            &Uuid::default()
        } else if job.structure_name.starts_with("SPBS") {
            &Uuid::default()
        } else if job.structure_name.starts_with("Q-0") {
            &Uuid::default()
        } else if job.structure_name == "UALX-3 - The Science Lounge" {
            &sqlx::query!("
                    SELECT id
                    FROM structure
                    WHERE id = '019d9e07-912f-708a-ae51-e89762d034e7'
                    AND owner = 2117441999
                ")
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
        } else if job.structure_name == "UALX-3 - GEZ T2 React Reproc" {
            &sqlx::query!("
                    SELECT id
                    FROM structure
                    WHERE id = '019da0ac-c5f8-734c-a9fe-a26af17ab2ee'
                    AND owner = 2117441999
                ")
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
        } else if job.structure_name == "ABE-M2 - Advanced Components" {
            &sqlx::query!("
                    SELECT id
                    FROM structure
                    WHERE id = '01980544-fcc0-7646-9831-24dd4f78073b'
                    AND owner = 2117441999
                ")
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
        } else if job.structure_name == "ABE-M2 - Caution Reactor" {
            &sqlx::query!("
                    SELECT id
                    FROM structure
                    WHERE id = '019814b5-9020-7a4c-8890-de895a69f736'
                    AND owner = 2117441999
                ")
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
        }else if job.structure_name == "ABE-M2 - Caution Reactor -Rami" {
            &sqlx::query!("
                    SELECT id
                    FROM structure
                    WHERE id = '019814b5-9020-7a4c-8890-de895a69f736'
                    AND owner = 2117441999
                ")
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
        } else {
            &sqlx::query!("
                    SELECT id
                    FROM structure
                    WHERE name = $1
                    AND owner = 2117441999
                ",
                    job.structure_name,
                )
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
        };

        mappings.insert(job.project_id, project_id.clone());
        mappings.insert(job.structure_id, structure_id.clone());
        visited_names.insert(job.project_name.clone());
        visited_names.insert(job.structure_name.clone());
    }

    for (index, job) in jobs.iter().enumerate() {
        tracing::info!("[{:6} / {:6}] Copying Jobs", index, jobs.len());
        let timestamp = Timestamp::from_unix(NoContext, job.created_at.timestamp() as u64, 0);
        let job_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&job.project_id) {
            x.clone()
        } else {
            tracing::error!("type id not found");
            continue;
        };
        let structure_id =  if let Some(x) = mappings.get(&job.structure_id) {
            x
        } else {
            tracing::error!("structure id not found");
            continue;
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

    /*let misc_entries = sqlx::query!(r#"
            SELECT
                pm.project_id,
                pm.item,
                pm.cost,
                pm.quantity,
                pm.description,
                pm.created_at,
                pm.updated_at,
                p.name AS project_name
            FROM project_misc pm
            JOIN project p ON p.id = pm.project_id
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_misc
        ")
        .execute(&mut *transaction)
        .await?;
    for (index, misc) in misc_entries.iter().enumerate() {
        tracing::info!("[{:6} / {:6}] Copying Misc", index, misc_entries.len());
        let timestamp = Timestamp::from_unix(NoContext, misc.created_at.timestamp() as u64, 0);
        let misc_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&misc.project_id) {
            x
        } else {
            if visited_names.contains(&misc.project_name) {
                continue;
            }

            &sqlx::query!("
                    SELECT id
                    FROM project
                    WHERE name = $1
                ",
                    misc.project_name,
                )
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
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

    for market in market_entries {
        let timestamp = Timestamp::from_unix(NoContext, market.created_at.timestamp() as u64, 0);
        let market_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&market.project_id.unwrap()) {
            x
        } else {
            tracing::info!("project_id not found - project_market");
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
    }
    tracing::info!("[project] project market migrated");

    let stock_entries = sqlx::query!(r#"
            SELECT
                ps.project_id,
                ps.type_id,
                ps.quantity,
                ps.cost,
                ps.created_at,
                ps.updated_at,
                p.name AS project_name
            FROM project_stock ps
            JOIN project p ON p.id = ps.project_id
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_stock
        ")
        .execute(&mut *transaction)
        .await?;
    for (index, stock) in stock_entries.iter().enumerate() {
        tracing::info!("[{:6} / {:6}] Copying Stock", index, stock_entries.len());
        let timestamp = Timestamp::from_unix(NoContext, stock.created_at.timestamp() as u64, 0);
        let stock_id = Uuid::new_v7(timestamp);
        let project_id =  if let Some(x) = mappings.get(&stock.project_id) {
            x
        } else {
            if visited_names.contains(&stock.project_name) {
                continue;
            }

            &sqlx::query!("
                    SELECT id
                    FROM project
                    WHERE name = $1
                ",
                    stock.project_name,
                )
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
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
    }
    tracing::info!("[project] project stock migrated");

    let excess_entries = sqlx::query!(r#"
            SELECT
                pe.project_id,
                pe.type_id,
                pe.quantity,
                pe.cost,
                pe.created_at,
                pe.updated_at,
                p.name AS project_name
            FROM project_excess pe
            JOIN project p ON p.id = pe.project_id
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_excess
        ")
        .execute(&mut *transaction)
        .await?;
    for (index, excess) in excess_entries.iter().enumerate() {
        tracing::info!("[{:6} / {:6}] Copying Excess", index, excess_entries.len());
        let project_id =  if let Some(x) = mappings.get(&excess.project_id) {
            x
        } else {
            if visited_names.contains(&excess.project_name) {
                continue;
            }

            &sqlx::query!("
                    SELECT id
                    FROM project
                    WHERE name = $1
                ",
                    excess.project_name,
                )
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
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
    }
    tracing::info!("[project] project excess migrated");

    let product_entries = sqlx::query!(r#"
            SELECT
                pp.id,
                pp.project_id,
                pp.type_id,
                pp.quantity,
                pp.material_efficiency,
                pp.created_at,
                pp.updated_at,
                p.name AS project_name
            FROM project_product pp
            JOIN project p ON p.id = pp.project_id
        "#)
        .fetch_all(postgres_source)
        .await?;
    sqlx::query!("
            DELETE FROM project_product
        ")
        .execute(&mut *transaction)
        .await?;
    for (index, product) in product_entries.iter().enumerate() {
        tracing::info!("[{:6} / {:6}] Copying Product", index, misc_entries.len());
        let project_id = if let Some(x) = mappings.get(&product.project_id) {
            x
        } else {
            if visited_names.contains(&product.project_name) {
                continue;
            }

            &sqlx::query!("
                    SELECT id
                    FROM project
                    WHERE name = $1
                ",
                    product.project_name,
                )
                .fetch_one(postgres_destination)
                .await
                .unwrap()
                .id
        };
        let project_id = mappings.get(&project_id).unwrap();

        sqlx::query!("
                INSERT INTO project_product (
                    id,
                    project_id,
                    type_id,
                    quantity,
                    material_efficiency,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7)
            ",
                product.id,
                project_id,
                product.type_id,
                product.quantity,
                product.material_efficiency,
                product.created_at,
                product.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }*/

    transaction.commit().await?;

    let fix_new_projects = vec![
        (Uuid::from_str("019d9cf2-fba8-7548-a11b-63d947e327bb").unwrap(), 23919),
        (Uuid::from_str("019d9d0b-b55f-7a7f-bad5-45938d4e7eb0").unwrap(), 20183),
        (Uuid::from_str("019d9d0c-7dc1-7bb9-bcbe-0a6c8d30aa76").unwrap(), 20183),
        (Uuid::from_str("019d9d0d-0387-7902-a35f-1a5b32dd9a33").unwrap(), 28606),
        (Uuid::from_str("019d9d0d-5424-7d51-9787-be38e3052639").unwrap(), 28606),
        (Uuid::from_str("019d9d0d-dfcd-7204-90ca-eb69aa3962f2").unwrap(), 28606),
        (Uuid::from_str("019d9d0e-9065-7b41-b2df-471a549ed70a").unwrap(), 37605),
        (Uuid::from_str("019d9d0f-0001-7469-a191-a6920eff4705").unwrap(), 73790),
        (Uuid::from_str("019d9d0f-9a70-7e6c-aa4a-a2b55d75c63e").unwrap(), 37604),
        (Uuid::from_str("019d9d10-042c-776a-aecf-5ece1ff8ea29").unwrap(), 52907),
        (Uuid::from_str("019d9d10-5a82-732a-95e0-a01719e62590").unwrap(), 23911),
        (Uuid::from_str("019d9d10-a839-7a05-ad05-c8483a978cb0").unwrap(), 23911),
        (Uuid::from_str("019d9d10-efae-793c-aad5-b9ca9e72fa9d").unwrap(), 73793),
        (Uuid::from_str("019d9d11-5712-7eeb-abc0-6917a0412fd7").unwrap(), 19726),
        (Uuid::from_str("019d9d11-adcd-7ad9-a972-387f53791585").unwrap(), 19726),
    ];
    for (project_id, product) in fix_new_projects {
        restore_new_jobs(postgres_destination, project_id).await;
        restore_product(postgres_destination, project_id, product).await;
    }
    tracing::info!("Done - project");

    Ok(())
}

async fn restore_new_jobs(
    pool:       &PgPool,
    project_id: Uuid,
) {
    sqlx::query!("
            INSERT INTO project_job
            (
                project_id,
                type_id,
                runs,
                structure_id
            )
            SELECT $1, * FROM (
                SELECT type_id, runs, structure_id
                FROM solution_manufacturing
                WHERE solution_id = (SELECT solution_id FROM project WHERE id = $1)
            );
        ",
            project_id,
        )
        .execute(pool)
        .await
        .unwrap();
}

async fn restore_product(
    pool:           &PgPool,
    project_id:     Uuid,
    type_id:        i32,
) {
    let solution_id = sqlx::query!("
            SELECT solution_id
            FROM project
            WHERE id = $1
        ",
            project_id,
        )
        .fetch_one(pool)
        .await
        .unwrap()
        .solution_id;

    sqlx::query!("
            INSERT INTO solution_product
            (
                solution_id,
                type_id,
                quantity,
                material_efficiency
            )
            VALUES ($1, $2, $3, $4)
        ",
            solution_id,
            type_id,
            1,
            8,
        )
        .execute(pool)
        .await
        .unwrap();
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
