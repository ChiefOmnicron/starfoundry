mod project_group;
mod structure;
mod structure_group;

pub use self::project_group::*;
pub use self::structure::*;
pub use self::structure_group::*;

use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;
use std::collections::HashMap;
use sqlx::PgPool;

type Mapping = HashMap<Uuid, Uuid>;

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
    let structure_group_mapping = migrate_structure_group(&postgres_source, &postgres_destination, &structure_mapping).await?;

    let project_group_mapping = migrate_project_group(&postgres_source, &postgres_destination, &structure_mapping).await?;

    Ok(())
}

async fn cleanup(
    postgres_destination: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!("
            DELETE FROM structure_group_structure CASCADE
        ")
        .execute(postgres_destination)
        .await?;
    sqlx::query!("
            DELETE FROM structure_group CASCADE
        ")
        .execute(postgres_destination)
        .await?;
    sqlx::query!("
            DELETE FROM structure CASCADE
        ")
        .execute(postgres_destination)
        .await?;

    sqlx::query!("
            DELETE FROM project_group CASCADE
        ")
        .execute(postgres_destination)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_member CASCADE
        ")
        .execute(postgres_destination)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_default_market CASCADE
        ")
        .execute(postgres_destination)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_default_blacklist CASCADE
        ")
        .execute(postgres_destination)
        .await?;

    Ok(())
}
