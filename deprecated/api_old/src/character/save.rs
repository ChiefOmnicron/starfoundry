use sqlx::PgPool;
use tracing::instrument;

use super::{Character, CharacterError};

/// Saves the character information in the database
///
/// # Params
///
/// * `character` -> All information about the character
/// * `main`      -> Optional main character id
///
#[instrument(err, skip(pool), level = "error")]
pub async fn save(
    pool:      &PgPool,
    character: &Character,
) -> Result<(), CharacterError> {
    sqlx::query!("
            INSERT INTO character
            (
                alliance_id, alliance_name,
                character_id, character_name,
                corporation_id, corporation_name
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (character_id)
            DO UPDATE SET
                alliance_id      = $1,
                alliance_name    = $2,
                corporation_id   = $5,
                corporation_name = $6
        ",
            character.alliance_id.map(|x| *x),
            character.alliance_name,
            *character.character_id as i32,
            character.character_name,
            *character.corporation_id as i32,
            character.corporation_name,
        )
        .execute(pool)
        .await
        .map(drop)
        .map_err(CharacterError::SaveCharacter)
}
