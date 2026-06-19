use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};
use utoipa::ToSchema;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "alliance_id": 99003214,
        "alliance_name": "Brave Collective",
        "character_id": 2117441999,
        "character_name": "Eistonen Kodan Sasen",
        "corporation_id": 98024275,
        "corporation_name": "Rational Chaos Inc."
    })
)]
pub struct CharacterInfo {
    pub character_name:   String,
    pub character_id:     CharacterId,

    pub corporation_name: String,
    pub corporation_id:   CorporationId,

    pub alliance_name:    Option<String>,
    pub alliance_id:      Option<AllianceId>,
}

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!({
        "alliance_id": 99003214,
        "alliance_name": "Brave Collective",
        "character_id": 2117441999,
        "character_name": "Eistonen Kodan Sasen",
        "corporation_id": 98024275,
        "corporation_name": "Rational Chaos Inc.",
        "scopes": ["publicData", "esi-industry.read_character_jobs.v1"]
    })
)]
pub struct AuthedCharacterInfo {
    pub character_name:   String,
    pub character_id:     CharacterId,

    pub corporation_name: String,
    pub corporation_id:   CorporationId,

    pub alliance_name:    Option<String>,
    pub alliance_id:      Option<AllianceId>,

    pub scopes:           Vec<String>,
}
