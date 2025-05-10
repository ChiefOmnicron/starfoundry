use sqlx::postgres::PgPoolOptions;
use starfoundry_bin_collector::execute;
use starfoundry_bin_collector::sql::ensure_tables;
use starfoundry_libs_eve_api::CredentialCache;
use std::sync::{Arc, Mutex};
use tracing_subscriber::EnvFilter;

/// ENV variable for the database URL
const PG_ADDR: &str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let pg_addr = std::env::var(PG_ADDR).expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .min_connections(25)
        .connect(&pg_addr)
        .await?;

    ensure_tables(&pool).await;

    let credential_cache = CredentialCache::load_from_database(&pool.clone()).await?;
    let credential_cache = Arc::new(Mutex::new(credential_cache));

    execute(pool, credential_cache).await?;
    
    Ok(())
}
