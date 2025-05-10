use sqlx::PgPool;
use starfoundry_libs_types::CharacterId;
use std::collections::HashMap;
use std::time::{Instant, Duration};

use crate::{Error, EveApiClient};
use std::sync::{Arc, Mutex};

/// Convenient type for the CredentialCache
pub type Credentials = Arc<Mutex<CredentialCache>>;

/// Wrapper for a [HashMap] where all values only live 12 hours. After that time
/// they are removed from the map.
/// 
/// When a new entry is inserted, the map is checked if the netries are still
/// valid.
/// 
#[derive(Clone, Debug)]
pub struct CredentialCache(HashMap<CharacterId, (EveApiClient, Instant)>);

impl CredentialCache {

    /// Sets the time before a token is no longer valid.
    /// 
    /// The usual time is 1200 seconds (20 minutes), in order to have some
    /// wiggle room, the timer is only set to 1140 seconds (19 minutes).
    /// 
    /// Seconds * Minutes
    /// 
    ///
    const LIFESPAN: Duration = Duration::new(60 * 19, 0);

    /// Creates an empty [CredentialCache].
    /// 
    /// See the [HashMap] for more information on the internal work.
    /// 
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    /// Loads all credentials from the database and puts them into the database
    /// 
    pub async fn load_from_database(
        pool: &PgPool,
    ) -> Result<Self, Error> {
        let mut _self = Self::new();
        _self.refresh(&pool).await?;

        Ok(_self)
    }

    /// Clears the cache and reloads all credentials from the database
    pub async fn refresh(
        &mut self,
        pool: &PgPool,
    ) -> Result<(), Error> {
        self.0 = HashMap::new();

        let credentials = sqlx::query!("
                SELECT
                    refresh_token,
                    c.character_id,
                    c.corporation_id
                FROM   credential cred
                JOIN   character c ON c.character_id = cred.character_id
                WHERE  cred.character_id IS NOT NULL
                AND  refresh_token IS NOT NULL
                AND  credential_type = 'CHARACTER'
            ")
            .fetch_all(pool)
            .await
            .unwrap();

        for credential in credentials {
            let client = EveApiClient::new_with_refresh_token(
                credential.character_id.into(),
                credential.corporation_id.into(),
                credential.refresh_token.unwrap(),
            ).unwrap();

            self.insert(
                credential.character_id.into(),
                client,
            );
        }

        let credentials = sqlx::query!(r#"
                SELECT
                    refresh_token,
                    character_id AS "character_id!"
                FROM   credential cred
                WHERE  character_id IS NOT NULL
                AND    refresh_token IS NOT NULL
                AND    credential_type = 'CORPORATION'
            "#)
            .fetch_all(pool)
            .await
            .unwrap();

        for credential in credentials {
            let client = EveApiClient::new_with_refresh_token(
                credential.character_id.into(),
                credential.character_id.into(),
                credential.refresh_token.unwrap(),
            ).unwrap();

            self.insert(
                credential.character_id.into(),
                client,
            );
        }

        // Empty client, that can be used for routes that do not require auth
        self.insert(
            CharacterId(0),
            EveApiClient::new()?,
        );

        Ok(())
    }

    /// Inserts the given value with the given key into the underlying map.
    /// 
    pub fn insert(
        &mut self,
        key:   CharacterId,
        value: EveApiClient,
    ) {
        let deadline = Instant::now() + Self::LIFESPAN;
        self.0.insert(key, (value, deadline));
    }

    /// Gets an entry from the underlying map.
    /// 
    pub async fn get(
        &self,
        key: CharacterId,
    ) -> Result<EveApiClient, Error> {
        let now = Instant::now();
        let (client, deadline) = self.0
            .get(&key)
            .ok_or(Error::NoSuchIdentity(key))?;

        if *deadline < now && key != CharacterId(0) {
            client.refresh_token().await?;
        }

        Ok(client.clone())
    }
}
