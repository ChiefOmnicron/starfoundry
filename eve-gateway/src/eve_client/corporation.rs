use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CorporationId};

use crate::eve_client::error::Result;
use crate::eve_client::EveApiClient;

impl EveApiClient {
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
            .fetch::<_, EveCorporationInfo>(&path, &())
            .await
            .map_err(Into::into)
    }
}

/// General information about the character
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EveCorporationInfo {
    /// Optional alliance id the character is in
    pub alliance_id: Option<AllianceId>,
    /// Name of the character
    pub name: String,
}
