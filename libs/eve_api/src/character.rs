use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CharacterId};

use crate::{BlueprintInfo, Cache, CorporationId, EveApiClient};
use crate::{AssetEntry, Error, ItemId};

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
    ) -> Result<CharacterInfo, Error> {
        let character_id = self.authenticated
            .as_ref()
            .ok_or(Error::ClientNotAuthenticated)?
            .character_id;

        let path = format!(
            "latest/characters/{}/",
            character_id,
        );

        self
            .fetch::<CharacterInfo>(&path, Cache::Ignore)
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
    /// TODO: remove duplicate - required for initial login
    pub async fn character_info_by_id(
        &self,
        character_id: CharacterId,
    ) -> Result<CharacterInfo, Error> {
        let path = format!(
            "latest/characters/{}/",
            character_id,
        );

        self
            .fetch::<CharacterInfo>(&path, Cache::Ignore)
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
    pub async fn alliance_name(
        &self,
        alliance_id: AllianceId,
    ) -> Result<String, Error> {
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
            .fetch::<Alliance>(&path, Cache::Ignore)
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
    ) -> Result<Vec<AssetEntry>, Error> {
        let character_id = self.authenticated
            .as_ref()
            .ok_or(Error::ClientNotAuthenticated)?
            .character_id;

        let path = format!(
            "latest/characters/{}/assets",
            character_id,
        );

        self
            .fetch_page_auth::<AssetEntry>(&path, Cache::Follow)
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
    ) -> Result<Vec<BlueprintInfo>, Error> {
        let character_id = self.authenticated
            .as_ref()
            .ok_or(Error::ClientNotAuthenticated)?
            .character_id;

        let path = format!(
            "latest/characters/{}/blueprints",
            character_id,
        );

        self
            .fetch_page_auth::<BlueprintInfo>(&path, Cache::Follow)
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
    ) -> Result<Vec<AssetName>, Error> {
        let character_id = self.authenticated
            .as_ref()
            .ok_or(Error::ClientNotAuthenticated)?
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
pub struct CharacterInfo {
    /// Optional alliance id the character is in
    pub alliance_id:    Option<AllianceId>,
    /// Corporation id of the character
    pub corporation_id: CorporationId,
    /// Name of the character
    pub name:           String,
}

/// Information about a location by [LocationId]
#[derive(Debug, Deserialize)]
pub struct AssetName {
    /// Id of the location id that maps to the name
    pub item_id: ItemId,
    /// Name of the location, for example a container or station
    pub name:    String,
}
