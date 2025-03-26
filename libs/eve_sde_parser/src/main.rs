//! Parses parts of the EVE provided SDE-File into SQL-Statements for the main
//! application.

use sqlx::postgres::PgPoolOptions;
use starfoundry_libs_eve_sde_parser::Error;
use std::time::Instant;
use tracing_subscriber::EnvFilter;

/// ENV variable for the database URL
const PG_ADDR: &str           = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pg_addr = std::env::var(PG_ADDR)
        .expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .min_connections(20)
        .connect(&pg_addr)
        .await
        .unwrap();

    let start = Instant::now();

    starfoundry_libs_eve_sde_parser::import_sde(&pool, None).await?;

    tracing::info!("Total run time: {}ms", start.elapsed().as_millis());

    Ok(())
}
