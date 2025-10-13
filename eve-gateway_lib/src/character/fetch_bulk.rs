use starfoundry_lib_types::CharacterId;

use crate::{CharacterInfo, ApiClient, Result};

pub async fn fetch_bulk_character(
    gateway_client: &impl ApiClient,
    character_ids:  Vec<CharacterId>,
) -> Result<Vec<CharacterInfo>> {
    gateway_client
        .post(&format!("characters/bulk"), character_ids)
        .await
        .map_err(Into::into)
}
