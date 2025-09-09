use sqlx::PgPool;
use starfoundry_lib_eve_api::{CredentialCache, EveApiClient};
use starfoundry_lib_types::{CharacterId, CorporationId};
use std::fmt::{self, Debug, Formatter};
use std::sync::{Arc, Mutex};
use crate::auth::error::{AuthError, Result};

/// Represents a logged in character.
#[derive(Clone)]
pub struct Identity {
    pool:             PgPool,
    credential_cache: Option<Arc<Mutex<CredentialCache>>>,
    character_id:     CharacterId,
}

impl Identity {
    /// Creates a new user instance.
    ///
    /// # Params
    ///
    /// * `token`        -> Token that is send by the cookie
    /// * `auth_service` -> Authentication service
    ///
    /// # Returns
    ///
    /// New user instance.
    ///
    pub fn new(
        pool:             PgPool,
        credential_cache: Arc<Mutex<CredentialCache>>,
        character_id:     CharacterId,
    ) -> Self {
        Self {
            pool,
            credential_cache: Some(credential_cache),
            character_id:     character_id.into(),
        }
    }

    pub fn new_without_credential_cache(
        pool:         PgPool,
        character_id: CharacterId,
    ) -> Self {
        Self {
            pool,
            credential_cache: None,
            character_id:     character_id.into(),
        }
    }

    /// Creates a new EVE-Authentication client for sending messages to the
    /// EVE-API that require that the user is logged in.
    ///
    /// # Errors
    ///
    /// Fails if getting a new refresh token from the API fails.
    ///
    /// # Returns
    ///
    /// A newly created authentication client, with a fresh token.
    ///
    pub async fn api_client(&self) -> Result<EveApiClient> {
        if let Some(credential_cache) = &self.credential_cache {
            let client = {
                credential_cache
                    .lock()
                    .unwrap()
                    .clone()
            };

            client
                .get(self.character_id)
                .await
                .map_err(|e| {
                    tracing::error!("{}", e);
                    AuthError::IdentityNotFound
                })
        } else {
            let corporation_id = self.corporation_id().await?;

            let refresh_token = self.refresh_token().await?;
            let client = EveApiClient::new_with_refresh_token(
                    self.character_id,
                    corporation_id,
                    refresh_token,
                )?;
            Ok(client)
        }
    }

    /// Gets the current refresh token for the identity
    /// 
    pub async fn refresh_token(
        &self,
    ) -> Result<String, AuthError> {
        let refresh_token = sqlx::query!("
                SELECT refresh_token
                FROM credential
                WHERE character_id = $1
                LIMIT 1
            ",
                *self.character_id,
            )
            .fetch_one(&self.pool)
            .await
            .map_err(|_| AuthError::InvalidIdentity)?
            .refresh_token
            .ok_or(AuthError::InvalidIdentity)?;
        Ok(refresh_token)
    }

    /// Gets the [CharacterId] of this identity
    ///
    pub fn character_id(
        &self,
    ) -> CharacterId {
        self.character_id
    }

    /// Gets the [CorporationId] of this identity
    ///
    pub async fn corporation_id(
        &self,
    ) -> Result<CorporationId> {
        let character = sqlx::query!("
                    SELECT corporation_id
                    FROM character
                    WHERE character_id = $1
                ",
                *self.character_id
            )
            .fetch_optional(&self.pool)
            .await
            .map_err(AuthError::GenericSqlxError)?;

        if let Some(x) = character {
            Ok(x.corporation_id.into())
        } else {
            Err(AuthError::InvalidIdentity)
        }
    }

    /// Gets its own identity and of the alts
    pub async fn character_identities(
        &self,
    ) -> Result<Vec<Identity>> {
        // With the condition that character_id must not be null, we know that
        // the id definitely exists
        let character_ids = sqlx::query!(r#"
                SELECT character_id AS "character_id!: CharacterId"
                FROM   credential
                WHERE
                    (character_id = $1 OR character_main = $1) AND
                    credential_type = 'CHARACTER' AND
                    character_id IS NOT NULL
            "#,
                *self.character_id,
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|_| AuthError::IdentityNotFound)?
            .into_iter()
            .map(|x| x.character_id)
            .collect::<Vec<_>>();

        let mut clients = Vec::new();
        for character_id in character_ids {
            clients.push(
                Identity::new_without_credential_cache(
                    self.pool.clone(),
                    character_id,
                )
            );
        }
        Ok(clients)
    }

    /// Gets its own identity and of the alts
    pub async fn corporation_identities(
        &self,
    ) -> Result<Vec<Identity>> {
        // With the condition that character_id must not be null, we know that
        // the id definitely exists
        let character_ids = sqlx::query!(r#"
                SELECT character_id AS "character_id!: CharacterId"
                FROM   credential
                WHERE
                    character_main = $1 AND
                    credential_type = 'CORPORATION' AND
                    character_id IS NOT NULL
            "#,
                *self.character_id,
            )
            .fetch_all(&self.pool)
            .await
            .map_err(|_| AuthError::IdentityNotFound)?
            .into_iter()
            .map(|x| x.character_id)
            .collect::<Vec<_>>();

        let mut clients = Vec::new();
        for character_id in character_ids {
            clients.push(
                Identity::new_without_credential_cache(
                    self.pool.clone(),
                    character_id,
                )
            );
        }
        Ok(clients)
    }
}

impl Debug for Identity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthUser")
            .field("character_id", &self.character_id)
            .field("token", &"***")
            .finish()
    }
}
