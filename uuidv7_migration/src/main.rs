mod project;
mod project_group;
mod structure;
mod structure_group;

pub use self::project::*;
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

    let environment = std::env::var("ENVIRONMENT").expect("'ENVIRONMENT' must be set");

    let postgres_source = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_SOURCE").unwrap())
        .await?;
    let postgres_destination_industry = PgPoolOptions::new()
        .connect(&std::env::var("DATABASE_DESTINATION_INDUSTRY").unwrap())
        .await?;

    let mut mappings = mappings(&postgres_source, environment.clone()).await;

    cleanup(&postgres_destination_industry).await?;
    cleanup_mapping(&postgres_source, environment.clone()).await;
    migrate_structure(&postgres_source, &postgres_destination_industry, &mut mappings).await?;
    migrate_industry_hubs(&postgres_source, &postgres_destination_industry, &mut mappings).await?;

    migrate_project_group(&postgres_source, &postgres_destination_industry, &mut mappings).await?;
    migrate_project(&postgres_source, &postgres_destination_industry, &mut mappings).await?;

    save_mappings(&postgres_source, environment, mappings).await;

    Ok(())
}

async fn mappings(
    postgres_source: &PgPool,
    environment:     String,
) -> Mapping {
    sqlx::query!("
            SELECT
                source_uuid,
                target_uuid
            FROM tmp_mappings
            WHERE environment = $1
        ",
            environment,
        )
        .fetch_all(postgres_source)
        .await
        .unwrap()
        .into_iter()
        .map(|x| (x.source_uuid, x.target_uuid))
        .collect::<HashMap<_, _>>()
}

async fn save_mappings(
    postgres_source: &PgPool,
    environment:     String,
    mappings:        Mapping,
) {
    let source = mappings
        .iter()
        .map(|(s, _)| s.clone())
        .collect::<Vec<_>>();
    let target = mappings
        .iter()
        .map(|(_, t)| t.clone())
        .collect::<Vec<_>>();

    sqlx::query!("
            INSERT INTO tmp_mappings
            (
                environment,
                source_uuid,
                target_uuid
            )
            SELECT $1, * FROM UNNEST(
                $2::UUID[],
                $3::UUID[]
            )
            ON CONFLICT (source_uuid) DO NOTHING
        ",
            environment,
            &source,
            &target,
        )
        .execute(postgres_source)
        .await
        .unwrap();
}

#[allow(dead_code)]
async fn cleanup(
    postgres_destination_industry: &PgPool,
) -> Result<(), Box<dyn std::error::Error>> {
    sqlx::query!("
            DELETE FROM project_job CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM project CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM industry_hub_structure CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM industry_hub CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM structure CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;

    sqlx::query!("
            DELETE FROM project_group CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_member CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_default_market CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_default_blacklist CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_default_blueprint_overwrite CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_default_job_splitting_general CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;
    sqlx::query!("
            DELETE FROM project_group_default_job_splitting_run CASCADE
        ")
        .execute(postgres_destination_industry)
        .await?;

    Ok(())
}

#[allow(dead_code)]
async fn cleanup_mapping(
    postgres_source: &PgPool,
    environment:     String,
) {
    sqlx::query!("
            DELETE FROM
            tmp_mappings
            WHERE environment = $1
        ",
            environment,
        )
        .execute(postgres_source)
        .await
        .unwrap();
}
