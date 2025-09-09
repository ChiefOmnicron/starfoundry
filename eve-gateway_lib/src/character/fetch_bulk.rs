use starfoundry_lib_types::CharacterId;

use crate::{CharacterInfo, EveGatewayClient, Result};

pub async fn fetch_bulk_character(
    gateway_client: &impl EveGatewayClient,
    character_ids:  Vec<CharacterId>,
) -> Result<Vec<CharacterInfo>> {
    gateway_client
        .post(&format!("characters/bulk"), character_ids)
        .await
        .map_err(Into::into)
}
