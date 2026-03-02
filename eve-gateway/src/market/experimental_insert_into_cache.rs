use sqlx::PgPool;

pub async fn write_to_cache(
    pool: &PgPool,
    data: serde_json::Value,
    hash: String,
) -> Result<(), ()> {
    sqlx::query!("
            INSERT INTO eve_cache (
                value,
                hash
            )
            VALUES ($1, $2)
            ON CONFLICT (hash)
            DO UPDATE SET
                value = EXCLUDED.value,
                created_at = NOW()
        ",
            data,
            hash,
        )
        .execute(pool)
        .await
        .unwrap();
    Ok(())
}
