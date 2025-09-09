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

    npc_stations(&pool).await?;

    Ok(())
}

async fn npc_stations(
    pool: &PgPool,
) -> Result<(), Error> {
    let stations = vec![
        (
            // uuid
            Uuid::from_str("00000000-0000-0000-0000-000000000001").unwrap(),
            // structure_id
            60003760,
            // system_id
            30000142,
            // type_id
            52678,
            // name
            "Jita 4-4"
        ),
        (
            // uuid
            Uuid::from_str("00000000-0000-0000-0000-000000000002").unwrap(),
            // structure_id
            60008494,
            // system_id
            30002187,
            // type_id
            1932,
            // name
            "Amarr"
        ),
    ];

    for (id, structure_id, system_id, type_id, name) in stations {
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
                VALUES ($1, $2, $3, $4, 'HIGHSEC', $5, 0, '{35892}')
            ",
                id,
                structure_id,
                system_id,
                type_id,
                name,
            )
            .execute(pool)
            .await
            .map_err(|e| Error::InsertStation(e, id))?;
        }
    }

    Ok(())
}
