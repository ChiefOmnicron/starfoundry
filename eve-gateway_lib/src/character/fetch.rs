use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};
use utoipa::ToSchema;

use crate::{EveGatewayClient, Result};

pub async fn fetch_character(
    gateway_client: &impl EveGatewayClient,
    character_id:   CharacterId,
) -> Result<CharacterInfo> {
    gateway_client
        .fetch(&format!("characters/{}", *character_id))
        .await
        .map_err(Into::into)
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct CharacterInfo {
    pub character_name:   String,
    pub character_id:     CharacterId,

    pub corporation_name: String,
    pub corporation_id:   CorporationId,

    pub alliance_name:    Option<String>,
    pub alliance_id:      Option<AllianceId>,
}
