use axum::http::{HeaderMap, HeaderValue};
use starfoundry_lib_types::{CharacterId, CorporationId};

use crate::{ExtractIdentity, HEADER_CHARACTER_ID, HEADER_CORPORATION_ID, HEADER_SOURCE};

#[derive(Clone)]
pub struct Identity {
    character_id:   CharacterId,
    corporation_id: CorporationId,
    source:         String,
}

impl Identity {
    pub fn new(
        character_id:   CharacterId,
        corporation_id: CorporationId,
        source:         String,
    ) -> Self {
        Self {
            character_id,
            corporation_id,
            source,
        }
    }

    pub fn character_id(&self) -> CharacterId {
        self.character_id
    }

    pub fn corporation_id(&self) -> CorporationId {
        self.corporation_id
    }

    pub fn as_header(
        &self,
    ) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            HEADER_SOURCE,
            HeaderValue::from_str(&self.source)
                .unwrap_or(HeaderValue::from_static("invalid.header"))
        );
        headers.insert(HEADER_CHARACTER_ID, (*self.character_id).into());
        headers.insert(HEADER_CORPORATION_ID, (*self.corporation_id).into());

        headers
    }
}

impl From<ExtractIdentity> for Identity {
    fn from(value: ExtractIdentity) -> Self {
        Self {
            character_id:   value.character_id,
            corporation_id: value.corporation_id,
            source:         value.host().unwrap_or_default(),
        }
    }
}
