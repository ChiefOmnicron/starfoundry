mod fetch;
mod fetch_bulk;

use serde::{Deserialize, Serialize};
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};
use utoipa::ToSchema;

pub use self::fetch::*;
pub use self::fetch_bulk::*;

#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
pub struct CharacterInfo {
    pub character_name:   String,
    pub character_id:     CharacterId,

    pub corporation_name: String,
    pub corporation_id:   CorporationId,

    pub alliance_name:    Option<String>,
    pub alliance_id:      Option<AllianceId>,
}
