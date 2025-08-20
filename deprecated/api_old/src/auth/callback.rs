use sqlx::PgPool;
use starfoundry_lib_eve_api::{CredentialCache, EveApiClient, EveJwtToken};
use std::sync::{Arc, Mutex};
use tracing::instrument;
use uuid::Uuid;

use super::AuthError;

/// state: Token that we generated and set, with that we can identify the request
#[instrument(level = "error", skip(pool))]
pub async fn callback(
    pool:             &PgPool,
    code:             &str,
    state:            &Uuid,
    credential_cache: Arc<Mutex<CredentialCache>>,
) -> Result<EveJwtToken, AuthError> {
    let character    = EveApiClient::access_token(code).await?;
    let character_id = character.character_id()?;

    let res = sqlx::query!("
            SELECT token, credential_type
            FROM   credential
            WHERE  token = $1
        ",
            state
        )
        .fetch_one(pool)
        .await
        .map_err(AuthError::CannotGetIntentionToken)?;

    if res.credential_type == "CORPORATION" {
        let corporation_id = sqlx::query!("
                SELECT corporation_id
                FROM character
                WHERE character_id = $1
                ",
                    *character_id,
                )
                .fetch_one(pool)
                .await
                .map_err(AuthError::CannotUpdateLogin)?
                .corporation_id;

        sqlx::query!("
            UPDATE credential
                SET
                    character_id   = $1,
                    refresh_token  = $2,
                    access_token   = $3
                WHERE token = $4
            ",
                corporation_id,
                &character.refresh_token,
                &character.access_token,
                state
            )
            .execute(pool)
            .await
            .map_err(AuthError::CannotUpdateLogin)?;
    } else {
        sqlx::query!("
            UPDATE credential
            SET
                character_id  = $1,
                refresh_token = $2,
                access_token  = $3
            WHERE token = $4
        ",
            *character_id,
            &character.refresh_token,
            &character.access_token,
            state
        )
        .execute(pool)
        .await
        .map_err(AuthError::CannotUpdateLogin)?;
    }

    let temp_client = EveApiClient::new().unwrap();
    let character_info = temp_client
        .character_info_by_id(character_id)
        .await
        .unwrap();

    let eve_client = EveApiClient::new_with_refresh_token(
        character_id,
        character_info.corporation_id,
        character.refresh_token.clone(),
    )?;

    credential_cache
        .lock()
        .unwrap()
        .insert(character_id, eve_client);

    Ok(character)
}
