mod error;
mod fetch;
mod fetch_bulk;
mod list;

use serde::{Deserialize, Serialize};
use starfoundry_gateway::ApiClient;
use starfoundry_lib_types::{AllianceId, CharacterId, CorporationId};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

pub(crate) use self::error::CharacterError;
pub(crate) use self::fetch::refresh_character_in_db;
use crate::Result;
use crate::state::AppState;

/// returns all routes that are under the path `/characters`
pub fn routes() -> OpenApiRouter<AppState> {
    let fetch = OpenApiRouter::new().routes(routes!(self::fetch::api));

    let fetch_bulk =
        OpenApiRouter::new().routes(routes!(self::fetch_bulk::api));

    let list = OpenApiRouter::new().routes(routes!(self::list::api));

    OpenApiRouter::new()
        .merge(fetch)
        .merge(fetch_bulk)
        .merge(list)
        // fittings
        .merge(crate::fitting::routes())
}

pub trait EveGatewayApiClientCharacter: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn list_characters(&self) -> Result<Option<AuthedCharacterInfo>> {
        self.fetch("characters", &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_character(
        &self,
        character_id: CharacterId,
    ) -> Result<Option<CharacterInfo>> {
        self.fetch(&format!("characters/{}", *character_id), &())
            .await
            .map_err(Into::into)
    }

    #[allow(async_fn_in_trait)]
    async fn fetch_character_bulk(
        &self,
        character_ids: Vec<CharacterId>,
    ) -> Result<Vec<CharacterInfo>> {
        self.post("characters/bulk", character_ids)
            .await
            .map_err(Into::into)
    }
}

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
    pub character_name: String,
    pub character_id:   CharacterId,

    pub corporation_name: String,
    pub corporation_id:   CorporationId,

    pub alliance_name: Option<String>,
    pub alliance_id:   Option<AllianceId>,
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
    pub character_name: String,
    pub character_id:   CharacterId,

    pub corporation_name: String,
    pub corporation_id:   CorporationId,

    pub alliance_name: Option<String>,
    pub alliance_id:   Option<AllianceId>,

    pub scopes: Vec<String>,
}
