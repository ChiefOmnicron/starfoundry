use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CorporationId, ItemId, LocationId, TypeId};

use crate::eve_client::error::{EveApiError, Result};
use crate::eve_client::EveApiClient;
use crate::eve_client::models::{AssetEntry, AssetName};

impl EveApiClient {
    /// Gets all assets the corporation owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of Blueprints
    ///
    pub async fn asset_names(
        &self,
        item_ids: Vec<ItemId>,
    ) -> Result<Vec<AssetName>> {
        let corporation_id = self.authenticated
            .as_ref()
            .ok_or(EveApiError::ClientNotAuthenticated)?
            .corporation_id;

        let path = format!(
            "latest/corporations/{}/assets/names",
            corporation_id,
        );
        self
            .post::<Vec<ItemId>, Vec<AssetName>>(item_ids, &path)
            .await
            .map_err(Into::into)
    }

    /// Gets general information about the corporation
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// Corporation information
    ///
    pub async fn corporation_info_by_id(
        &self,
        corporation_id: CorporationId,
    ) -> Result<EveCorporationInfo> {
        let path = format!(
            "latest/corporations/{}",
            corporation_id,
        );

        self
            .fetch::<EveCorporationInfo>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets all assets the corporation owns
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of Blueprints
    ///
    pub async fn corporation_assets(
        &self,
        corporation_id: CorporationId,
    ) -> Result<Vec<AssetEntry>> {
        let path = format!(
            "latest/corporations/{}/assets",
            corporation_id,
        );

        self
            .fetch_page_auth::<AssetEntry>(&path)
            .await
            .map_err(Into::into)
    }

    /// Gets a list of names for the given [LocationId].
    ///
    /// # Limits
    ///
    /// This is only for the current corporation as determined by the
    /// [EveAuthClient].
    ///
    /// # Params
    ///
    /// * `client` > Authenticated ESI client
    /// * `lid`    > List of [LocationId]s to resolve
    ///
    /// # Errors
    ///
    /// - If the endpoint is not available
    /// - If the response cannot be parsed
    ///
    /// # Returns
    ///
    /// List of [Location]s that match the given [LocationId].
    ///
    pub async fn location_name(
        &self,
        corporation_id: CorporationId,
        location_ids:   Vec<LocationId>,
    ) -> Result<Vec<ItemLocation>> {
        let path = format!(
            "latest/corporations/{}/assets/names",
            corporation_id,
        );

        self
            .post::<Vec<LocationId>, Vec<ItemLocation>>(location_ids, &path)
            .await
            .map_err(Into::into)
    }

    /// Gets a list of all blueprints the corporation owns.
    ///
    /// # Limits
    ///
    /// This is only for the current corporation as determined by the
    /// [EveAuthClient].
    ///
    /// # Params
    ///
    /// * `client` > Authenticated ESI client
    ///
    /// # Errors
    ///
    /// - If the endpoint is not available
    /// - If the response cannot be parsed
    ///
    /// # Returns
    ///
    /// List of all blueprints
    ///
    pub async fn corporation_blueprints(
        &self,
    ) -> Result<Vec<BlueprintInfo>> {
        let corporation_id = self.authenticated
            .as_ref()
            .ok_or(EveApiError::ClientNotAuthenticated)?
            .corporation_id;

        let path = format!(
            "latest/corporations/{}/blueprints",
            corporation_id,
        );

        self
            .fetch_page_auth::<BlueprintInfo>(&path)
            .await
            .map_err(Into::into)
    }
}

/// Represents a transaction entry
#[derive(Debug, Deserialize)]
pub struct JournalEntry {
    /// ISK amount
    pub amount:          f32,
    /// Balance of the wallet after the transaction
    pub balance:         f32,
    /// Date the transaction was performed
    pub date:            String,
    /// Information about the transaction
    pub description:     String,
    /// Unique ID
    pub id:              u64,
    /// Either a [CharacterId] or [CorporationId]
    pub first_party_id:  u64,
    /// Either a [CharacterId] or [CorporationId]
    pub second_party_id: u64,
    /// Type of journal entry
    pub ref_type:        String,

    /// Depends on [JournalEntry::ref_type]
    pub context_id:      Option<u64>,
    /// [CorporationId] that is the receiver from bounty_prices
    pub tax_receiver_id: Option<u64>,
    /// Reason of the entry
    pub reason:          Option<String>,
}

/// Represents a wallet entry
#[derive(Debug, Deserialize)]
pub struct WalletEntry {
    /// Current balance of the division
    pub balance:  f32,
    /// Devision number, eg: 1 is the master wallet
    pub division: u8,
}

/// General information about the character
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EveCorporationInfo {
    /// Optional alliance id the character is in
    pub alliance_id: Option<AllianceId>,
    /// Name of the character
    pub name: String,
}

/// Information about a location by [LocationId]
#[derive(Debug, Deserialize)]
pub struct ItemLocation {
    /// Id of the location id that maps to the name
    pub item_id: LocationId,
    /// Name of the location, for example a container or station
    pub name: String,
}

/// Information of a blueprint
#[derive(Clone, Debug, Deserialize)]
pub struct BlueprintInfo {
    /// Unique EVE Item ID
    pub item_id:             ItemId,
    /// Location of the blueprint
    pub location_flag:       String,
    /// Location ID of the structure the blueprint is in
    pub location_id:         LocationId,
    /// Material efficiency of the blueprint
    pub material_efficiency: u32,
    /// range of numbers with a minimum of -2 and no maximum value where -1 is
    /// an original and -2 is a copy. It can be a positive integer if it is a
    /// stack of blueprint originals fresh from the market (e.g. no activities performed on them yet).
    pub quantity:            i32,
    /// Number of runs remaining if the blueprint is a copy, -1 if it is an original.
    pub runs:                i32,
    /// Time efficiency of the blueprint
    pub time_efficiency:     u32,
    /// TypeID of the blueprint
    pub type_id:             TypeId,
}
