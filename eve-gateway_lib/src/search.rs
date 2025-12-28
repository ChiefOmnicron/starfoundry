use axum::http::{HeaderMap, HeaderValue};
use reqwest::header::HOST;
use serde::{Deserialize, Serialize};
use starfoundry_lib_gateway::{ApiClient, HEADER_CHARACTER_ID};
use starfoundry_lib_types::CharacterId;
use utoipa::ToSchema;

use crate::{Error, Result};

/// One of:
/// - agent
/// - alliance
/// - character
/// - constellation
/// - corporation
/// - faction
/// - inventory_type
/// - region
/// - solar_system
/// - station
/// - structure
/// 
#[derive(Clone, Debug, Deserialize, Serialize, ToSchema)]
#[schema(
    example = json!([])
)]
pub struct SearchResult(pub Vec<i64>);

pub trait EveGatewayApiClientSearch: ApiClient {
    #[allow(async_fn_in_trait)]
    async fn fetch_market_by_region(
        &self,
        character_id: CharacterId,
        source:       String,
        category:     SearchCategory,
        search:       String,
    ) -> Result<SearchResult> {
        let mut headers = HeaderMap::new();
        headers.insert(HOST, HeaderValue::from_str(&source).unwrap_or(HeaderValue::from_static("invalid.header")));
        headers.insert(HEADER_CHARACTER_ID, (*character_id).into());

        let category_str: String = category.clone().into();
        let query = &[
            ("categories", category_str.as_ref()),
            ("search", search.as_ref())
        ];

        self
            .fetch_auth(
                &format!("characters/{}/search", *character_id),
                query,
                headers,
            )
            .await
            .map_err(Into::into)
            .map(|x: EveSearchResult| {
                let result = match category {
                    SearchCategory::Agent         => x.agent,
                    SearchCategory::Alliance      => x.alliance,
                    SearchCategory::Character     => x.character,
                    SearchCategory::Constellation => x.constellation,
                    SearchCategory::Corporation   => x.corporation,
                    SearchCategory::Faction       => x.faction,
                    SearchCategory::InventoryType => x.inventory_type,
                    SearchCategory::Region        => x.region,
                    SearchCategory::SolarSystem   => x.solar_system,
                    SearchCategory::Station       => x.station,
                    SearchCategory::Structure     => x.structure,
                };
                SearchResult(result)
            })
    }
}

#[derive(Clone, Debug, Deserialize)]
pub enum SearchCategory {
    Agent,
    Alliance,
    Character,
    Constellation,
    Corporation,
    Faction,
    InventoryType,
    Region,
    SolarSystem,
    Station,
    Structure,
}

impl TryFrom<String> for SearchCategory {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_ref() {
            "agent"          => Ok(Self::Agent),
            "alliance"       => Ok(Self::Alliance),
            "character"      => Ok(Self::Character),
            "constellation"  => Ok(Self::Constellation),
            "corporation"    => Ok(Self::Corporation),
            "faction"        => Ok(Self::Faction),
            "inventory_type" => Ok(Self::InventoryType),
            "region"         => Ok(Self::Region),
            "solar_system"   => Ok(Self::SolarSystem),
            "station"        => Ok(Self::Station),
            "structure"      => Ok(Self::Structure),
            _                => Err(Error::InvalidSearchCategory(value))
        }
    }
}

impl Into<String> for SearchCategory {
    fn into(self) -> String {
        match self {
            Self::Agent         => "agent",
            Self::Alliance      => "alliance",
            Self::Character     => "character",
            Self::Constellation => "constellation",
            Self::Corporation   => "corporation",
            Self::Faction       => "faction",
            Self::InventoryType => "inventory_type",
            Self::Region        => "region",
            Self::SolarSystem   => "solar_system",
            Self::Station       => "station",
            Self::Structure     => "structure",
        }.into()
    }
}

#[derive(Deserialize)]
struct EveSearchResult {
    #[serde(default)]
    agent:          Vec<i64>,
    #[serde(default)]
    alliance:       Vec<i64>,
    #[serde(default)]
    character:      Vec<i64>,
    #[serde(default)]
    constellation:  Vec<i64>,
    #[serde(default)]
    corporation:    Vec<i64>,
    #[serde(default)]
    faction:        Vec<i64>,
    #[serde(default)]
    inventory_type: Vec<i64>,
    #[serde(default)]
    region:         Vec<i64>,
    #[serde(default)]
    solar_system:   Vec<i64>,
    #[serde(default)]
    station:        Vec<i64>,
    #[serde(default)]
    structure:      Vec<i64>,
}
