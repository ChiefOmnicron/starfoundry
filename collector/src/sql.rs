use sqlx::PgPool;

pub async fn ensure_tables(
    pool: &PgPool,
) {
    wallet_character(pool).await;
    wallet_corporation(pool).await;
}

async fn wallet_character(
    pool: &PgPool,
) {
    sqlx::query!("
        CREATE TABLE IF NOT EXISTS wallet_character(
            id          BIGINT  NOT NULL,
            receiver    BIGINT  NOT NULL,
            sender      BIGINT  NOT NULL,

            character   BIGINT  NOT NULL,

            amount      FLOAT   NOT NULL,
            balance     FLOAT   NOT NULL,

            date        VARCHAR NOT NULL,
            ref_type    VARCHAR NOT NULL,

            reason      VARCHAR,
            context_id  BIGINT,

            PRIMARY KEY(id),
            FOREIGN KEY (character)
                REFERENCES characters(character_id)
                ON DELETE CASCADE
        );
    ")
    .execute(pool)
    .await
    .unwrap();
}

async fn wallet_corporation(
    pool: &PgPool,
) {
    sqlx::query!("
        CREATE TABLE IF NOT EXISTS wallet_corporation(
            id          BIGINT  NOT NULL,
            receiver    BIGINT  NOT NULL,
            sender      BIGINT  NOT NULL,

            corporation BIGINT  NOT NULL,
            division    INTEGER NOT NULL,

            amount      FLOAT   NOT NULL,
            balance     FLOAT   NOT NULL,

            date        VARCHAR NOT NULL,
            ref_type    VARCHAR NOT NULL,

            reason      VARCHAR,
            context_id  BIGINT,

            PRIMARY KEY(id)
        );
    ")
    .execute(pool)
    .await
    .unwrap();
}
