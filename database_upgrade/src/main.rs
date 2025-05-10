mod error;

use std::str::FromStr;

use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

pub use self::error::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pg_addr = std::env::var("DATABASE_URL").expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .connect(&pg_addr)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    default_user(&pool).await?;
    default_project_group(&pool).await?;
    default_project_group_member(&pool).await?;
    npc_stations(&pool).await?;

    Ok(())
}

async fn default_user(
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query!("
            INSERT INTO character (
                character_id,
                corporation_id,
                character_name,
                corporation_name
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
        ",
            0,
            0,
            "Default Character",
            "Default Corporation",
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(Error::InsertDefaultUser)
}

async fn default_project_group(
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query!("
            INSERT INTO project_group (
                id,
                owner,
                name,
                description
            )
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
        ",
            Uuid::default(),
            0,
            "Default",
            "Default Group",
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(Error::InsertDefaultProjectGroup)
}

async fn default_project_group_member(
    pool: &PgPool,
) -> Result<(), Error> {
    sqlx::query!("
            INSERT INTO project_group_member (
                group_id,
                character_id,
                accepted,
                projects,
                structures
            )
            VALUES ($1, $2, $3, $4, $5)
            ON CONFLICT DO NOTHING
        ",
            Uuid::default(),
            0,
            true,
            "WRITE",
            "WRITE",
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(Error::InsertDefaultProjectGroupMember)
}

async fn npc_stations(
    pool: &PgPool,
) -> Result<(), Error> {
    let stations = vec![
        (Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(), 60003760, 30000142, "Jita 4-4"),
        (Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(), 60008494, 30002187, "Amarr"),
    ];

    for (id, structure_id, system_id, name) in stations {
        let exists = sqlx::query!("
                SELECT 1 AS exists
                FROM structure
                WHERE id = $1
            ",
                id,
            )
            .fetch_optional(pool)
            .await
            .map_err(|e| Error::FetchStation(e, id))?;

        if let None = exists {
            sqlx::query!("
                INSERT INTO structure (
                    id,
                    structure_id,
                    system_id,
                    type_id,
                    security,
                    name,
                    owner,
                    services
                )
                VALUES ($1, $2, $3, 0, 'HIGHSEC', $4, 0, '{35878}')
            ",
                id,
                structure_id,
                system_id,
                name,
            )
            .execute(pool)
            .await
            .map_err(|e| Error::InsertStation(e, id))?;
        }
    }

    Ok(())
}
