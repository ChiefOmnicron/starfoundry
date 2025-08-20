use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;
use tracing::instrument;

use super::{Character, CharacterError};

/// Gets a list of alts for the given [CharacterId]
///
/// # Params
///
/// * `cid` -> [CharacterId] of the requesting character
///
/// # Returns
///
/// List of alt characters
///
#[instrument(err, skip(pool), level = "error")]
pub async fn corporations(
    pool:         &PgPool,
    character_id: CharacterId,
) -> Result<Vec<Character>, CharacterError> {
    let corporations = sqlx::query!(r#"
            SELECT DISTINCT character_id AS "character_id!: CharacterId"
            FROM   credential
            WHERE character_main = $1
            AND credential_type = 'CORPORATION'
        "#,
            *character_id as i32
        )
        .fetch_all(pool)
        .await
        .map_err(CharacterError::FetchAlts)?;

    let mut characters = Vec::new();
    for corporation in corporations {
        let mut character = super::service::info_corporation(
            &pool,
            corporation.character_id,
        ).await?;
        character.credential_type = "CORPORATION".into();
        characters.push(character);
    }

    Ok(characters)
}
