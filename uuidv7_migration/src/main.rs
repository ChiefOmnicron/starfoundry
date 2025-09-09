use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use std::collections::HashMap;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv()?;

    let postgres_source = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_SOURCE").unwrap())
        .await?;
    let postgres_destination = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_DESC").unwrap())
        .await?;

    cleanup(&postgres_destination).await?;

    let project_group_mappings = migrate_project_group(&postgres_source, &postgres_destination).await?;
    let structure_group_mappings = migrate_structure_group(&postgres_source, &postgres_destination).await?;
    let project_mappings = migrate_project(&postgres_source, &postgres_destination, &project_group_mappings, &structure_group_mappings).await?;
    migrate_project_blacklist(&postgres_source, &postgres_destination, &project_mappings).await?;
    migrate_project_excess(&postgres_source, &postgres_destination, &project_mappings).await?;

    Ok(())
}

async fn cleanup(
    postgres_destination: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!("
            DELETE FROM project CASCADE
        ")
        .execute(postgres_destination)
        .await?;
    sqlx::query!("
            DELETE FROM project_group CASCADE
        ")
        .execute(postgres_destination)
        .await?;
    sqlx::query!("
            DELETE FROM structure_group CASCADE
        ")
        .execute(postgres_destination)
        .await?;

    Ok(())
}

async fn migrate_project_group(
    postgres_source: &PgPool,
    postgres_destination: &PgPool,
) -> Result<HashMap<Uuid, Uuid>, Box<dyn std::error::Error>> {
    dbg!("Start - project_group");
    let project_groups = sqlx::query!(r#"
            SELECT
                id,
                owner,
                name,
                description,
                created_at,
                updated_at
            FROM project_group
            ORDER BY created_at
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut id_mapping = HashMap::new();
    let mut transaction = postgres_destination.begin().await?;
    for project_group in project_groups {
        if project_group.id.get_version_num() == 7 {
            continue;
        }

        let uuid_v7 = if project_group.id.is_nil() {
            Uuid::default()
        } else {
            Uuid::now_v7()
        };
        id_mapping.insert(project_group.id, uuid_v7);

        sqlx::query!("
                INSERT INTO project_group
                (
                    id,
                    owner,
                    name,
                    description,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
                uuid_v7,
                project_group.owner,
                project_group.name,
                project_group.description,
                project_group.created_at,
                project_group.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    dbg!("Done - project_group");

    Ok(id_mapping)
}

async fn migrate_project(
    postgres_source: &PgPool,
    postgres_destination: &PgPool,
    project_group_mappings: &HashMap<Uuid, Uuid>,
    structure_group_mappings: &HashMap<Uuid, Uuid>,
) -> Result<HashMap<Uuid, Uuid>, Box<dyn std::error::Error>> {
    dbg!("Start - project");
    let projects = sqlx::query!(r#"
            SELECT
                id,
                owner,
                name,
                status AS "status: String",
                sell_price,
                orderer,
                note,
                structure_group_id,
                project_group_id,
                created_at,
                updated_at
            FROM project
            ORDER BY created_at
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut id_mapping = HashMap::new();
    let mut transaction = postgres_destination.begin().await?;
    for project in projects {
        if project.id.get_version_num() == 7 {
            continue;
        }

        let uuid_v7 = Uuid::now_v7();
        id_mapping.insert(project.id, uuid_v7);

        let structure_group_id = if project.structure_group_id.is_nil() {
            Uuid::default()
        } else {
            structure_group_mappings.get(&project.structure_group_id).unwrap().clone()
        };

        sqlx::query!("
                INSERT INTO project
                (
                    id,
                    owner,
                    name,
                    status,
                    sell_price,
                    orderer,
                    note,
                    structure_group_id,
                    project_group_id,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4::PROJECT_STATUS, $5, $6, $7, $8, $9, $10, $11)
            ",
                uuid_v7,
                project.owner,
                project.name,
                project.status as _,
                project.sell_price,
                project.orderer,
                project.note,
                structure_group_id,
                project_group_mappings.get(&project.project_group_id).unwrap(),
                project.created_at,
                project.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    dbg!("Start - project");

    Ok(id_mapping)
}

async fn migrate_structure_group(
    postgres_source: &PgPool,
    postgres_destination: &PgPool,
) -> Result<HashMap<Uuid, Uuid>, Box<dyn std::error::Error>> {
    dbg!("Start - structure_group");
    let structure_groups = sqlx::query!(r#"
            SELECT
                id,
                owner,
                name,
                created_at,
                updated_at
            FROM structure_group
            ORDER BY created_at
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut id_mapping = HashMap::new();
    let mut transaction = postgres_destination.begin().await?;
    for structure_group in structure_groups {
        if structure_group.id.get_version_num() == 7 {
            continue;
        }

        let uuid_v7 = if structure_group.id.is_nil() {
            Uuid::default()
        } else {
            Uuid::now_v7()
        };
        id_mapping.insert(structure_group.id, uuid_v7);

        sqlx::query!("
                INSERT INTO structure_group
                (
                    id,
                    owner,
                    name,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5)
            ",
                uuid_v7,
                structure_group.owner,
                structure_group.name,
                structure_group.created_at,
                structure_group.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;

    sqlx::query!("
            INSERT INTO structure_group
            (
                id,
                owner,
                name,
                created_at,
                updated_at
            )
            VALUES ($1, $2, $3, '1970-01-01 00:00:00' AT TIME ZONE 'UTC', '1970-01-01 00:00:00' AT TIME ZONE 'UTC')
        ",
            Uuid::default(),
            0,
            "Dummy Group",
        )
        .execute(postgres_destination)
        .await?;
    id_mapping.insert(Uuid::default(), Uuid::default());
    dbg!("Start - structure_group");

    Ok(id_mapping)
}

async fn migrate_project_blacklist(
    postgres_source: &PgPool,
    postgres_destination: &PgPool,
    project_mappings: &HashMap<Uuid, Uuid>
) -> Result<(), Box<dyn std::error::Error>> {
    dbg!("Start - project_blacklist");
    let project_blacklist = sqlx::query!(r#"
            SELECT
                project_id,
                type_id,
                created_at,
                updated_at
            FROM project_blacklist
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for blacklist in project_blacklist {
        if blacklist.project_id.get_version_num() == 7 {
            continue;
        }

        sqlx::query!("
                INSERT INTO project_blacklist
                (
                    project_id,
                    type_id,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4)
            ",
                project_mappings.get(&blacklist.project_id).unwrap(),
                blacklist.type_id,
                blacklist.created_at,
                blacklist.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    dbg!("Start - project_blacklist");
    Ok(())
}

async fn migrate_project_excess(
    postgres_source: &PgPool,
    postgres_destination: &PgPool,
    project_mappings: &HashMap<Uuid, Uuid>
) -> Result<(), Box<dyn std::error::Error>> {
    dbg!("Start - project_excess");
    let project_excess = sqlx::query!(r#"
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

    let mut transaction = postgres_destination.begin().await?;
    for excess in project_excess {
        if excess.project_id.get_version_num() == 7 {
            continue;
        }

        sqlx::query!("
                INSERT INTO project_excess
                (
                    project_id,
                    type_id,
                    quantity,
                    cost,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6)
            ",
                project_mappings.get(&excess.project_id).unwrap(),
                excess.type_id,
                excess.quantity,
                excess.cost,
                excess.created_at,
                excess.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    dbg!("Start - project_excess");
    Ok(())
}
