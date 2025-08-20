pub static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../../deprecated/database_upgrade/migrations");
