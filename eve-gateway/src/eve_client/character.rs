use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId, ItemId};

use crate::eve_client::corporation::BlueprintInfo;
use crate::eve_client::EveApiClient;
use crate::eve_client::models::{AssetEntry, AssetName};
use crate::eve_client::error::{EveApiError, Result};

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
    #[deprecated(note = "Use EveApiClient::character_info_by_id")]
    pub async fn character_info(
        &self,
    ) -> Result<EveCharacterInfo> {
        let character_id = self.authenticated
            .as_ref()
            .ok_or(EveApiError::ClientNotAuthenticated)?
            .character_id;

        let path = format!(
            "latest/characters/{}/",
            character_id,
        );

        self
            .fetch::<EveCharacterInfo>(&path)
            .await
            .map_err(Into::into)
    }

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
    /// TODO: rename to character_info once the old function is deleted
    pub async fn character_info_by_id(
        &self,
        character_id: CharacterId,
    ) -> Result<EveCharacterInfo> {
        let path = format!(
            "latest/characters/{}/",
            character_id,
        );

        self
            .fetch::<EveCharacterInfo>(&path)
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
            .fetch::<Alliance>(&path)
            .await
            .map(|x| x.name)
            .map_err(Into::into)
    }

    /// Gets all assets the character owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of assets
    ///
    pub async fn character_assets(
        &self,
    ) -> Result<Vec<AssetEntry>> {
        let character_id = self.authenticated
            .as_ref()
            .ok_or(EveApiError::ClientNotAuthenticated)?
            .character_id;

        let path = format!(
            "latest/characters/{}/assets",
            character_id,
        );

        self
            .fetch_page_auth::<AssetEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets all blueprints the character owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of blueprints
    ///
    pub async fn character_blueprints(
        &self,
    ) -> Result<Vec<BlueprintInfo>> {
        let character_id = self.authenticated
            .as_ref()
            .ok_or(EveApiError::ClientNotAuthenticated)?
            .character_id;

        let path = format!(
            "latest/characters/{}/blueprints",
            character_id,
        );

        self
            .fetch_page_auth::<BlueprintInfo>(path)
            .await
            .map_err(Into::into)
    }

    /// Gets all asset names of the assets the character owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of assets
    ///
    pub async fn character_asset_names(
        &self,
        item_ids: Vec<ItemId>,
    ) -> Result<Vec<AssetName>> {
        let character_id = self.authenticated
            .as_ref()
            .ok_or(EveApiError::ClientNotAuthenticated)?
            .character_id;

        let path = format!(
            "latest/characters/{}/assets/names",
            character_id,
        );

        self
            .post::<Vec<ItemId>, Vec<AssetName>>(item_ids, &path)
            .await
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
