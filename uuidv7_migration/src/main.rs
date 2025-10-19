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
        .connect(&std::env::var("DATABASE_DESTINATION").unwrap())
        .await?;

    cleanup(&postgres_destination).await?;

    let structure_mapping = migrate_structure(&postgres_source, &postgres_destination).await?;

    Ok(())
}

async fn cleanup(
    postgres_destination: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!("
            DELETE FROM structure CASCADE
        ")
        .execute(postgres_destination)
        .await?;

    Ok(())
}

async fn migrate_structure(
    postgres_source: &PgPool,
    postgres_destination: &PgPool,
) -> Result<HashMap<Uuid, Uuid>, Box<dyn std::error::Error>> {
    let mut mappings = HashMap::new();

    dbg!("Start - structure");
    let structures = sqlx::query!(r#"
            SELECT
                id,
                structure_id,
                system_id,
                type_id,
                rigs,
                services,
                name,
                owner,
                created_at,
                updated_at
            FROM structure
        "#)
        .fetch_all(postgres_source)
        .await?;

    let mut transaction = postgres_destination.begin().await?;
    for structure in structures {
        let structure_id = Uuid::now_v7();
        mappings.insert(structure.id, structure_id);

        sqlx::query!("
                INSERT INTO structure
                (
                    id,
                    structure_id,
                    system_id,
                    type_id,
                    rigs,
                    services,
                    name,
                    owner,
                    created_at,
                    updated_at
                )
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ",
                structure_id,
                structure.structure_id,
                structure.system_id,
                structure.type_id,
                &structure.rigs,
                &structure.services,
                structure.name,
                structure.owner,
                structure.created_at,
                structure.updated_at,
            )
            .execute(&mut *transaction)
            .await?;
    }
    transaction.commit().await?;
    dbg!("Done - structure");
    Ok(mappings)
}
