use std::collections::HashMap;

use sqlx::PgPool;
use starfoundry_lib_types::CharacterId;

use crate::character::error::{CharacterError, Result};
use crate::character::{refresh_character_in_db, CharacterInfo};

/// Fetches the character information for the given ids from the database.
/// If the character does not exist yet, it will be fetched using the EVE-API.
/// 
pub async fn fetch_bulk(
    pool:          &PgPool,
    character_ids: Vec<CharacterId>,
) -> Result<Vec<CharacterInfo>> {
    let mut db_lookup_result = sqlx::query!("
            SELECT
                character_id,
                character_name,
                corporation_id,
                corporation_name,
                alliance_id,
                alliance_name
            FROM character
            WHERE character_id = ANY($1)
        ",
            &character_ids.clone().into_iter().map(|x| *x).collect::<Vec<_>>(),
        )
        .fetch_all(pool)
        .await
        .map_err(CharacterError::FetchCharacterBulk)?
        .into_iter()
        .map(|x| (x.character_id, CharacterInfo {
            character_name:   x.character_name,
            character_id:     x.character_id.into(),
            corporation_name: x.corporation_name,
            corporation_id:   x.corporation_id.into(),
            alliance_name:    x.alliance_name,
            alliance_id:      x.alliance_id.map(Into::into),
        }))
        .collect::<HashMap<_, _>>();

    if db_lookup_result.len() != character_ids.len() {
        for character_id in character_ids {
            if !db_lookup_result.contains_key(&*character_id) {
                // ignore errors
                if let Ok(x) = refresh_character_in_db(&pool, character_id).await {
                    db_lookup_result.insert(*character_id, x);
                }
            }
        }
    }

    Ok(
        db_lookup_result
            .values()
            .cloned()
            .collect::<Vec<_>>()
        )
}
