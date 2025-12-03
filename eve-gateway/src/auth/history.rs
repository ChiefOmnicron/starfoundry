use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::auth::error::{AuthError, Result};

pub async fn insert_into_history(
    pool:           &PgPool,
    character_id:   CharacterId,
    source:         String,
) -> Result<()> {
    sqlx::query!("
            INSERT INTO login_history
            (
                character_id,
                source
            )
            VALUES ($1, $2)
        ",
            *character_id,
            source
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(AuthError::InsertLoginHistory)
}
