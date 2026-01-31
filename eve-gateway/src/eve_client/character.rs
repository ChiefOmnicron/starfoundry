use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};

use crate::eve_client::error::Result;
use crate::eve_client::EveApiClient;

impl EveApiClient {
    /// Gets general information about the character
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// Character information
    ///
    pub async fn character_info(
        &self,
        character_id: CharacterId,
    ) -> Result<EveCharacterInfo> {
        let path = format!(
            "latest/characters/{}/",
            character_id,
        );

        self
            .fetch::<_, EveCharacterInfo>(&path, &())
            .await
            .map_err(Into::into)
    }

    /// Gets the name of an alliance by its id
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// Alliance name
    ///
    pub async fn alliance_name_by_id(
        &self,
        alliance_id: AllianceId,
    ) -> Result<String> {
        /// Temporary struct for deserializing
        #[derive(Deserialize)]
        struct Alliance {
            /// Name of the alliance
            name: String,
        }

        let path = format!(
            "latest/alliances/{}",
            alliance_id,
        );

        self
            .fetch::<_, Alliance>(&path, &())
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }
}

/// General information about the character
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EveCharacterInfo {
    /// Optional alliance id the character is in
    pub alliance_id:    Option<AllianceId>,
    /// Corporation id of the character
    pub corporation_id: CorporationId,
    /// Name of the character
    pub name:           String,
}
