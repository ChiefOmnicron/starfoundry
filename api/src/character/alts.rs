use sqlx::PgPool;
use starfoundry_libs_eve_api::EveApiClient;
use starfoundry_libs_types::CharacterId;
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
pub async fn alts(
    pool:         &PgPool,
    client:       &EveApiClient,
    character_id: CharacterId,
) -> Result<Vec<Character>, CharacterError> {
    let alts = sqlx::query!(r#"
            SELECT DISTINCT character_id AS "character_id!: CharacterId",
                   character_main        AS "character_main: CharacterId",
                   credential_type
            FROM   credentials
            WHERE  character_main = $1
                AND character_id IS NOT NULL
                AND credential_type = 'CHARACTER'
        "#,
            *character_id as i32
        )
        .fetch_all(pool)
        .await
        .map_err(CharacterError::FetchAlts)?;

    let mut characters = Vec::new();
    for alt in alts {
        let character = super::service::info(
            &pool,
            client,
            alt.character_id,
        ).await?;
        characters.push(character);
    }

    Ok(characters)
}
