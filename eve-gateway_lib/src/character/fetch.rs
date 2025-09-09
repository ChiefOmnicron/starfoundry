use starfoundry_lib_types::CharacterId;

use crate::{CharacterInfo, EveGatewayClient, Result};

pub async fn fetch_character(
    gateway_client: &impl EveGatewayClient,
    character_id:   CharacterId,
) -> Result<CharacterInfo> {
    gateway_client
        .fetch(&format!("characters/{}", *character_id))
        .await
        .map_err(Into::into)
}

