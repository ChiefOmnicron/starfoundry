use serde::Serialize;
use starfoundry_libs_types::{AllianceId, CharacterId, CorporationId};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Character {
    pub id:               Uuid,
    pub character_name:   String,
    pub character_id:     CharacterId,
    pub corporation_name: String,
    pub corporation_id:   CorporationId,

    pub alliance_name:    Option<String>,
    pub alliance_id:      Option<AllianceId>,

    pub credential_type:  String,
}

impl Character {
    pub fn new(
        id:               Uuid,
        character_name:   String,
        character_id:     CharacterId,
        corporation_name: String,
        corporation_id:   CorporationId,

        alliance_name:    Option<String>,
        alliance_id:      Option<AllianceId>,

        credential_type:  String,
    ) -> Self {
        Self {
            id,
            character_name,
            character_id,
            corporation_name,
            corporation_id,
            alliance_name,
            alliance_id,
            credential_type,
        }
    }
}
