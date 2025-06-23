pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../database_upgrade/migrations");
