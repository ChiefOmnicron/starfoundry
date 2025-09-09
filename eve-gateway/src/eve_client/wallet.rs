use serde::Deserialize;

use crate::{Cache, Error, EveApiClient};

impl EveApiClient {
    /// Fetches market information for the given [RegionId].
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [EveClient] is not valid
    /// - If the character does not have access to the structure
    /// - If the structure does not exist
    /// - If the [RegionId] is not a valid id
    /// 
    /// # Returns
    /// Information about the structure
    /// 
    pub async fn wallet_character(
        &self,
    ) -> Result<Vec<JournalEntry>, Error> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(Error::ClientNotAuthenticated);
        };

        let path = format!(
            "latest/characters/{}/wallet/journal",
            authenticated.character_id,
        );

        let response = self
            .fetch_page_auth::<JournalEntry>(&path, Cache::Follow)
            .await
            .map_err(Into::into)?;

        Ok(response)
    }

    /// Fetches market information for the given [RegionId].
    /// 
    /// # Errors
    /// 
    /// - If the EVE API is not available
    /// - If the [EveClient] is not valid
    /// - If the character does not have access to the structure
    /// - If the structure does not exist
    /// - If the [RegionId] is not a valid id
    /// 
    /// # Returns
    /// Information about the structure
    /// 
    pub async fn wallet_corporation(
        &self,
        division: u8,
    ) -> Result<Vec<JournalEntry>, Error> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(Error::ClientNotAuthenticated);
        };

        let path = format!(
            "latest/corporations/{}/wallets/{}/journal",
            authenticated.corporation_id,
            division,
        );

        let response = self
            .fetch_page_auth::<JournalEntry>(&path, Cache::Follow)
            .await
            .map_err(Into::into)?;

        Ok(response)
    }

    /// Gets a list of all wallets and their current balance
    ///
    /// # Errors
    ///
    /// Fails when the server returns an error or parsing the response fails
    ///
    /// # Returns
    ///
    /// List of balance
    ///
    pub async fn wallets(&self) -> Result<Vec<WalletEntry>, Error> {
        let authenticated = if let Some(x) = &self.authenticated {
            x
        } else {
            return Err(Error::ClientNotAuthenticated);
        };

        let path = format!(
            "latest/corporations/{}/wallets",
            authenticated.corporation_id,
        );

        self
            .fetch_page_auth::<WalletEntry>(&path, Cache::Follow)
            .await
            .map_err(Into::into)
    }
}

/// Represents a wallet entry
#[derive(Debug, Deserialize)]
pub struct WalletEntry {
    /// Current balance of the division
    pub balance:  f32,
    /// Devision number, eg: 1 is the master wallet
    pub division: u8,
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
