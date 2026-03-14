use sqlx::PgPool;
use starfoundry_lib_eve_client::EveApiClientMetric;
use starfoundry_lib_eve_gateway::{AuthedCharacterInfo};
use starfoundry_lib_types::CharacterId;
use std::collections::HashMap;
use std::sync::Arc;

use crate::auth::error::{AuthError, Result};
use crate::character::service::fetch_character_bulk;

/// Fetches the character information for the given ids from the database.
/// If the character does not exist yet, it will be fetched using the EVE-API.
/// 
pub async fn list_authed_characters(
    pool:           &PgPool,
    metric:         Arc<EveApiClientMetric>,
    character_main: CharacterId,
    host:           String,
) -> Result<Vec<AuthedCharacterInfo>> {
    let characters_db = sqlx::query!("
            SELECT
                character_id,
                scopes
            FROM eve_credential
            WHERE
                (
                    character_id = $1 OR
                    character_main = $1
                ) AND
                domain = $2
        ",
            *character_main,
            host,
        )
        .fetch_all(pool)
        .await
        .map_err(AuthError::InsertRefreshToken)?
        .into_iter()
        .map(|x| (x.character_id.into(), x.scopes))
        .collect::<HashMap<_, _>>();

    let character_ids = characters_db
        .keys()
        .cloned()
        .collect::<Vec<_>>();

    let characters = fetch_character_bulk(
            pool,
            metric,
            character_ids,
        )
        .await?
        .into_iter()
        .map(|x| (x.character_id, x))
        .collect::<HashMap<_, _>>();

    let mut result = Vec::new();
    for (character_id, scopes) in characters_db {
        let info = if let Some(x) = characters.get(&character_id) {
            x
        } else {
            continue;
        };

        result.push(AuthedCharacterInfo {
            alliance_id:        info.alliance_id,
            alliance_name:      info.alliance_name.clone(),
            character_id:       info.character_id,
            character_name:     info.character_name.clone(),
            corporation_id:     info.corporation_id,
            corporation_name:   info.corporation_name.clone(),
            scopes:             scopes.clone().unwrap_or_default(),
        });
    }
    result.sort_by_key(|x| x.character_name.clone());

    Ok(result)
}
