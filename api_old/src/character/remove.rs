use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use tracing::instrument;

use super::CharacterError;

#[instrument(err, skip(pool), level = "error")]
pub async fn remove(
    pool:         &PgPool,
    character_id: CharacterId
) -> Result<(), CharacterError> {
    sqlx::query!("
            DELETE FROM character WHERE character_id = $1
        ",
            *character_id
        )
        .execute(pool)
        .await
        .map_err(CharacterError::RemoveCharacter)?;
    sqlx::query!("
            DELETE FROM character WHERE character_id = $1
        ",
            *character_id
        )
        .execute(pool)
        .await
        .map_err(CharacterError::RemoveCharacterLogin)?;
    Ok(())
}
