use sqlx::postgres::PgPoolOptions;


/// ENV variable for the database URL
const PG_ADDR: &str = "DATABASE_URL";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let pg_addr = std::env::var(PG_ADDR).expect("Expected that a DATABASE_URL ENV is set");
    let pool = PgPoolOptions::new()
        .connect(&pg_addr)
        .await?;
    sqlx::migrate!().run(&pool).await?;

    // TODO add if they don't exist
    // INSERT INTO event_queue (task) VALUES ('ASSET_CHECK');
    // INSERT INTO event_queue (task) VALUES ('CLEANUP_CHECK');
    // INSERT INTO event_queue (task) VALUES ('INDUSTRY_CHECK');
    // INSERT INTO event_queue (task) VALUES ('MARKET_CHECK');
    // INSERT INTO event_queue (task) VALUES ('SDE_CHECK');
    // INSERT INTO event_queue (task) VALUES ('STOCK_CHECK');

    // TODO add if they don't exist
    sqlx::query!("
            INSERT INTO structures (
                id,
                structure_id,
                system_id,
                type_id,
                security,
                name,
                owner
            )
            VALUES
            ('00000000-0000-0000-0000-000000000001', 60003760, 30000142, 0, 'HIGHSEC', 'Jita 4-4', 0),
            ('00000000-0000-0000-0000-000000000002', 60008494, 30002187, 0, 'HIGHSEC', 'Amarr', 0)
        ")
        .execute(&pool)
        .await
        .unwrap();

    Ok(())
}
