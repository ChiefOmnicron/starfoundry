use sqlx::PgPool;
use starfoundry_lib_eve_api::EveApiClient;
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};
use tracing::instrument;

use super::{Character, CharacterError};

#[instrument(err, skip(pool), level = "error")]
pub async fn info(
    pool:         &PgPool,
    client:       &EveApiClient,
    character_id: CharacterId,
) -> Result<Character, CharacterError> {
    #[cfg(feature = "test")]
    {
        if *character_id > 0 && *character_id <= 100 {
            return Ok(Character {
                id:                uuid::Uuid::new_v4(),
                character_name:   "Test Character".into(),
                character_id:     character_id,
                corporation_name: "Test Corporation".into(),
                corporation_id:   CorporationId(1),
                alliance_name:    None,
                alliance_id:      None,
                credential_type:  "CHARACTER".into(),
            });
        }
    };

    let character = sqlx::query_as!(
        Character,
        r#"
            SELECT
                c.id,
                c.alliance_id             AS "alliance_id: AllianceId",
                c.alliance_name,
                c.character_id            AS "character_id!: CharacterId",
                c.character_name,
                c.corporation_id          AS "corporation_id!: CorporationId",
                c.corporation_name,
                auth.credential_type
            FROM  credential auth
            JOIN  character c ON c.character_id = auth.character_id
            WHERE c.character_id = $1
              AND auth.credential_type = 'CHARACTER'
        "#,
            *character_id,
        )
        .fetch_optional(pool)
        .await
        .map_err(CharacterError::FetchCharacter)?;

    if let Some(x) = character {
        Ok(x)
    } else {
        let character = client
            .character_info()
            .await
            .map_err(CharacterError::FetchCharacterCoporation)?;

        let aid = character.alliance_id;
        let alliance = if let Some(x) = aid {
            let alliance_name = client
                .alliance_name(x)
                .await
                .map_err(CharacterError::FetchCharacterAlliance)?;
            Some(alliance_name)
        } else {
            None
        };

        let corporation = client
            .corporation_info(character.corporation_id)
            .await
            .map_err(CharacterError::FetchCharacterCoporation)?
            .name;

        let character = Character::new(
            uuid::Uuid::new_v4(),
            character.name,
            character_id,
            corporation,
            character.corporation_id,
            alliance,
            aid,
            "CHARACTER".into(),
        );

        super::service::save(&pool, &character).await?;
        Ok(character)
    }
}

#[instrument(err, skip(pool), level = "error")]
pub async fn info_corporation(
    pool:         &PgPool,
    character_id: CharacterId,
) -> Result<Character, CharacterError> {
    let corporation = sqlx::query_as!(
        Character,
        r#"
            SELECT
                c.id,
                c.alliance_id             AS "alliance_id: AllianceId",
                c.alliance_name,
                c.character_id            AS "character_id!: CharacterId",
                c.character_name,
                c.corporation_id          AS "corporation_id!: CorporationId",
                c.corporation_name,
                auth.credential_type
            FROM  credential auth
            JOIN  character c ON c.corporation_id = auth.character_id
            WHERE auth.character_id = $1 
              AND auth.credential_type = 'CORPORATION'
        "#,
            *character_id,
        )
        .fetch_one(pool)
        .await
        .map_err(CharacterError::FetchCharacter)?;

    let character = Character::new(
        uuid::Uuid::new_v4(),
        corporation.corporation_name.clone(),
        CharacterId(*corporation.corporation_id),
        corporation.corporation_name,
        corporation.corporation_id,
        corporation.alliance_name,
        corporation.alliance_id,
        "CORPORATION".into(),
    );

    Ok(character)
}
