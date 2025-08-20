use sqlx::PgPool;
use starfoundry_lib_eve_api::{CredentialCache, Credentials};
use std::sync::{Arc, Mutex};

pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../database_upgrade/migrations");

pub async fn credential_cache(
    pool: PgPool,
) -> Credentials {
    let credential_cache = CredentialCache::load_from_database(&pool.clone())
        .await
        .unwrap();
    Arc::new(Mutex::new(credential_cache))
}
